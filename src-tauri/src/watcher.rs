use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

/// 自身保存后的静默窗口：窗口期内发生的修改事件视为本次保存产生，忽略。
/// Windows 上杀毒软件扫描会延迟文件事件的到达，窗口过短会误报「文件被外部修改」。
const GRACE: Duration = Duration::from_millis(800);
/// 自身保存记录的最大保留时间（超过即清理）。
const OWN_WRITE_TTL: Duration = Duration::from_secs(1);
/// 广播给前端的 Tauri 事件名。
const EVENT: &str = "file-changed";

struct Shared {
    /// 被监视的父目录 -> 引用计数（多个打开文件可共用同一目录监视）
    dirs: HashMap<PathBuf, usize>,
    /// 当前打开文件的规范化绝对路径集合
    files: HashSet<PathBuf>,
    /// 自身写入时间表（用于 grace 静默期）
    own_writes: HashMap<PathBuf, Instant>,
}

#[derive(Clone)]
pub struct FileWatcherState {
    watcher: Arc<Mutex<RecommendedWatcher>>,
    shared: Arc<Mutex<Shared>>,
}

/// 把路径规范化为统一标识符（绝对化 + canonicalize + Windows 前缀修剪等）。
fn norm(p: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(crate::pathutil::canonicalize_path(&p.as_ref().to_string_lossy()))
}

impl FileWatcherState {
    pub fn new(app: AppHandle) -> Self {
        let shared = Arc::new(Mutex::new(Shared {
            dirs: HashMap::new(),
            files: HashSet::new(),
            own_writes: HashMap::new(),
        }));

        let emit_app = app.clone();
        let emit_shared = Arc::clone(&shared);
        let watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                Self::handle_event(&emit_app, &emit_shared, event);
            }
        })
        .expect("failed to create file watcher");

        Self {
            watcher: Arc::new(Mutex::new(watcher)),
            shared,
        }
    }

    /// 开始监视某个打开的文件（自动监视其所在目录，带引用计数）。
    pub fn watch_file(&self, path: &str) -> Result<(), String> {
        let file = norm(path);
        let dir = file
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| norm("."));

        let mut guard = self.shared.lock().unwrap();
        if guard.files.insert(file) {
            let count = guard.dirs.entry(dir.clone()).or_insert(0);
            if *count == 0 {
                let mut watcher = self.watcher.lock().unwrap();
                watcher
                    .watch(&dir, RecursiveMode::NonRecursive)
                    .map_err(|e| format!("开始监视目录失败: {e}"))?;
            }
            *count += 1;
        }
        Ok(())
    }

    /// 停止监视某个已关闭的文件。
    pub fn unwatch_file(&self, path: &str) {
        let file = norm(path);
        let dir = file
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| norm("."));

        let mut guard = self.shared.lock().unwrap();
        if guard.files.remove(&file) {
            if let Some(count) = guard.dirs.get_mut(&dir) {
                *count -= 1;
                if *count == 0 {
                    guard.dirs.remove(&dir);
                    let _ = self.watcher.lock().unwrap().unwatch(&dir);
                }
            }
        }
    }

    /// 记录一次自身写入，进入 grace 静默期。
    pub fn mark_own_write(&self, path: &str) {
        let file = norm(path);
        let mut guard = self.shared.lock().unwrap();
        guard.own_writes.insert(file, Instant::now());
        guard
            .own_writes
            .retain(|_, t| t.elapsed() < OWN_WRITE_TTL);
    }

    fn handle_event(app: &AppHandle, shared: &Arc<Mutex<Shared>>, event: notify::Event) {
        if matches!(event.kind, notify::EventKind::Access(_)) {
            return;
        }
        let guard = shared.lock().unwrap();
        let mut target: Option<PathBuf> = None;
        for p in &event.paths {
            if guard.files.contains(&norm(p)) {
                target = Some(PathBuf::from(p));
                break;
            }
        }
        let Some(target) = target else { return };
        if let Some(w) = guard.own_writes.get(&target) {
            if w.elapsed() < GRACE {
                return;
            }
        }
        drop(guard);
        let _ = app.emit(EVENT, target.to_string_lossy().into_owned());
    }
}

#[tauri::command]
pub fn watch_file(state: tauri::State<'_, FileWatcherState>, path: String) -> Result<(), String> {
    state.watch_file(&path)
}

#[tauri::command]
pub fn unwatch_file(state: tauri::State<'_, FileWatcherState>, path: String) {
    state.unwatch_file(&path)
}
