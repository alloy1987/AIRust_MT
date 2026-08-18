use serde::Serialize;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

use crate::encoding;
use crate::large_file::{self, MarkdownPreview};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub children: Option<Vec<TreeEntry>>,
}

/// 打开/另存为对话框的扩展名过滤器（与 large_file.rs 的白名单对应；md 在最前作为默认扩展名）
const TEXT_DIALOG_EXTENSIONS: &[&str] = &[
    "md", "markdown", "mdown", "mkd", "mkdn",
    "txt", "html", "htm", "css", "rtf",
    "json", "yaml", "yml", "xml", "toml", "ini", "csv",
    "py", "js", "mjs", "cjs", "jsx", "ts", "tsx", "java", "c", "h", "cpp", "hpp", "go", "rs",
    "env",
];

/// 读取文件并生成预览（大文件自动截断为前 max_bytes 字节）。
fn read_file_payload(path: &str) -> Result<MarkdownPreview, String> {
    let canonical = crate::pathutil::canonicalize_path(path);
    large_file::build_markdown_preview(&canonical, large_file::DEFAULT_MAX_BYTES)
}

/// 把 `spawn_blocking` 的 `JoinError` 归一化成字符串错误。
async fn block<T: Send + 'static>(f: impl FnOnce() -> Result<T, String> + Send + 'static) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(f)
        .await
        .unwrap_or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub async fn open_file_dialog(app: tauri::AppHandle) -> Result<Option<MarkdownPreview>, String> {
    block(move || {
        let picker = app
            .dialog()
            .file()
            .add_filter("文本文件", TEXT_DIALOG_EXTENSIONS)
            .set_title("打开文本文件")
            .blocking_pick_file();
        match picker {
            Some(path) => read_file_payload(&path.to_string()).map(Some),
            None => Ok(None),
        }
    })
    .await
}

#[tauri::command]
pub async fn open_folder_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    block(move || {
        let picker = app.dialog().file().blocking_pick_folder();
        Ok(picker.map(|p| crate::pathutil::canonicalize_path(&p.to_string()).to_string_lossy().into_owned()))
    })
    .await
}

/// 打开「选择图片」对话框，返回选中的图片绝对路径。
#[tauri::command]
pub async fn open_image_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    block(move || {
        let picker = app
            .dialog()
            .file()
            .add_filter("图片文件", &["png", "jpg", "jpeg", "gif", "bmp", "svg", "webp"])
            .set_title("选择图片")
            .blocking_pick_file();
        Ok(picker.map(|p| p.to_string()))
    })
    .await
}

#[tauri::command]
pub async fn read_file(path: String) -> Result<MarkdownPreview, String> {
    block(move || read_file_payload(&path)).await
}

/// 打开文件并生成预览，可自定义截断上限（默认 5 MB）。
#[tauri::command]
pub async fn open_markdown_preview(path: String, max_bytes: Option<u64>) -> Result<MarkdownPreview, String> {
    let max_bytes = max_bytes.unwrap_or(large_file::DEFAULT_MAX_BYTES);
    block(move || {
        // 与 read_file 保持一致：返回规范化路径，保证前端「同路径去重」与 watcher 标识统一
        let canonical = crate::pathutil::canonicalize_path(&path);
        large_file::build_markdown_preview(&canonical, max_bytes)
    })
    .await
}

/// 异步读取完整文件（保存大文件前必须先调用）。
#[tauri::command]
pub async fn ensure_full_content(path: String) -> Result<String, String> {
    large_file::ensure_full_content(path).await
}

/// 原子写：临时文件 + 落盘 + rename 替换。
///
/// - 拒绝只读文件
/// - `create_new` 防止并发冲突
/// - 进程内序号避免时间精度导致的重名
fn save_file_content(path: &str, bytes: &[u8]) -> Result<(), String> {
    // 进程内序号（避免 Windows 时间精度问题导致临时文件名重复）
    static TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

    let target = Path::new(path);

    // 拒绝只读文件
    if let Ok(meta) = std::fs::metadata(target) {
        if meta.permissions().readonly() {
            return Err(format!("{} 是只读文件，无法保存", target.display()));
        }
    }

    let dir = target
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let file_name = target.file_name().unwrap_or_default().to_string_lossy();
    let pid = std::process::id();
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    let seq = TEMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let tmp_path = dir.join(format!(".{file_name}.{pid}.{ts}.{seq}.tmp"));

    let result = (|| -> Result<(), String> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true) // create_new（防止并发冲突）
            .open(&tmp_path)
            .map_err(|e| format!("创建临时文件失败: {e}"))?;

        // 复制权限
        if let Ok(meta) = std::fs::metadata(target) {
            let _ = file.set_permissions(meta.permissions());
        }

        file.write_all(bytes).map_err(|e| format!("写入临时文件失败: {e}"))?;
        file.sync_all().map_err(|e| format!("刷新到磁盘失败: {e}"))?;
        drop(file);

        std::fs::rename(&tmp_path, target).map_err(|e| format!("替换原文件失败: {e}"))?;

        #[cfg(unix)]
        if let Ok(d) = std::fs::File::open(dir) {
            let _ = d.sync_all();
        }
        Ok(())
    })();

    if result.is_err() {
        let _ = std::fs::remove_file(&tmp_path);
    }
    result
}

#[tauri::command]
pub async fn save_file(
    path: String,
    content: String,
    encoding: String,
    watcher: tauri::State<'_, crate::watcher::FileWatcherState>,
) -> Result<(), String> {
    let canonical = crate::pathutil::canonicalize_path(&path);
    let canonical_str = canonical.to_string_lossy().into_owned();
    let watcher = (&*watcher).clone();
    block(move || {
        encoding::verify_lossless_roundtrip(&content, &encoding)?;
        let bytes = encoding::encode_text(&content, &encoding)?;
        // 写入前先进入 grace 静默期：本次写盘产生的监视事件将被忽略
        watcher.mark_own_write(&canonical_str);
        save_file_content(&canonical_str, &bytes)?;
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn save_file_as_dialog(
    app: tauri::AppHandle,
    watcher: tauri::State<'_, crate::watcher::FileWatcherState>,
    content: String,
    suggested_name: String,
) -> Result<Option<String>, String> {
    let watcher = (&*watcher).clone();
    block(move || {
        let picker = app
            .dialog()
            .file()
            .add_filter("文本文件", TEXT_DIALOG_EXTENSIONS)
            .set_file_name(suggested_name)
            .blocking_save_file();
        match picker {
            Some(path) => {
                let canonical = crate::pathutil::canonicalize_path(&path.to_string());
                let canonical_str = canonical.to_string_lossy().into_owned();
                encoding::verify_lossless_roundtrip(&content, "utf8")?;
                let bytes = encoding::encode_text(&content, "utf8")?;
                watcher.mark_own_write(&canonical_str);
                save_file_content(&canonical_str, &bytes)?;
                // 落盘后返回规范化路径，保证「另存为」得到统一标识符
                Ok(Some(canonical_str))
            }
            None => Ok(None),
        }
    })
    .await
}

#[tauri::command]
pub async fn list_dir(path: String, depth: usize) -> Result<Vec<TreeEntry>, String> {
    fn walk(dir: &Path, depth: usize, out: &mut Vec<TreeEntry>) {
        if depth == 0 {
            return;
        }
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        let mut list: Vec<TreeEntry> = Vec::new();
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            // 只隐藏会拖垮目录树的巨型/隐藏目录（node_modules、target、.git），其余全部显示
            if name == "node_modules" || name == "target" || name == ".git" {
                continue;
            }
            let path = entry.path();
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let mut child = TreeEntry {
                name,
                path: path.to_string_lossy().into_owned(),
                is_dir,
                children: None,
            };
            if is_dir {
                let mut subs = Vec::new();
                walk(&path, depth - 1, &mut subs);
                if !subs.is_empty() {
                    child.children = Some(subs);
                }
            }
            list.push(child);
        }
        list.sort_by(|a, b| {
            b.is_dir.cmp(&a.is_dir).then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        out.extend(list);
    }
    block(move || {
        let root = Path::new(&path);
        // 根目录读取失败时显式报错（否则侧栏会把权限不足等误显示为「空文件夹」）
        if let Err(e) = std::fs::read_dir(root) {
            return Err(format!("读取目录失败 {}: {e}", root.display()));
        }
        let mut out = Vec::new();
        walk(root, depth, &mut out);
        Ok(out)
    })
    .await
}

#[tauri::command]
pub async fn read_binary_file(path: String) -> Result<Vec<u8>, String> {
    block(move || std::fs::read(&path).map_err(|e| format!("读取文件失败: {e}"))).await
}

#[tauri::command]
pub async fn save_image(app: tauri::AppHandle, bytes: Vec<u8>, doc_dir: String, extension: String) -> Result<String, String> {
    block(move || {
        let dir = if doc_dir.is_empty() {
            app.path().app_data_dir().map_err(|e| e.to_string())?.join("images")
        } else {
            std::path::Path::new(&doc_dir).join("images")
        };
        std::fs::create_dir_all(&dir).map_err(|e| format!("创建图片目录失败: {e}"))?;

        // 扩展名只允许字母数字：防止 `..` / 路径分隔符借扩展名逃逸出 images 目录
        let ext: String = extension
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .take(10)
            .collect();
        let ext = if ext.is_empty() { "png".to_string() } else { ext };
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
        let name = format!("img_{ts}.{ext}");
        let target = dir.join(&name);
        std::fs::write(&target, bytes).map_err(|e| format!("保存图片失败: {e}"))?;
        if doc_dir.is_empty() {
            // 未命名文档：没有可锚定的文档目录，返回绝对路径（正斜杠），
            // 保证 markdown 引用无论文档之后保存到哪都能解析到实际文件。
            Ok(target.to_string_lossy().replace('\\', "/"))
        } else {
            Ok(format!("images/{name}"))
        }
    })
    .await
}
