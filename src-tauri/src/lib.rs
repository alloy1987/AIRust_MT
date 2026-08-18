mod commands;
mod encoding;
mod large_file;
mod menu;
mod pathutil;
mod settings;
mod watcher;

use std::sync::Mutex;
use tauri::Emitter;
use tauri::Manager;
use tauri::WindowEvent;

struct InitialFile(Mutex<Option<String>>);

#[tauri::command]
fn take_initial_file(state: tauri::State<'_, InitialFile>) -> Result<Option<String>, String> {
    let mut guard = state.0.lock().unwrap();
    Ok(guard.take())
}

/// 前端确认后真正退出应用（关闭按钮/菜单退出/确认对话框都走这里）。
#[tauri::command]
fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            // 第二次启动：把命令行里的文件路径转给已运行实例
            let file = args.iter().enumerate().find(|(i, a)| *i > 0 && !a.starts_with('-'));
            if let Some((_, p)) = file {
                let _ = app.emit("open-file", p.clone());
            }
            let w = app.get_webview_window("main");
            // 恢复最小化并置前聚焦，避免重复启动悄无声息
            if let Some(w) = w {
                let _ = w.unminimize();
                let _ = w.set_focus();
            }
        }))
        .on_window_event(|window, event| {
            // 拦截窗口关闭：先阻止默认关闭，交给前端检查未保存标签后再真正退出
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.emit("app-close-requested", ());
            }
        })
        .setup(|app| {
            let cli_file = std::env::args().skip(1).find(|a| !a.starts_with('-'));
            app.manage(InitialFile(Mutex::new(cli_file)));
            app.manage(watcher::FileWatcherState::new(app.handle().clone()));
            menu::build(app.handle())?;
            Ok(())
        })
        .on_menu_event(|app, event| {
            menu::handle(app, event.id().as_ref());
        })
        .invoke_handler(tauri::generate_handler![
            take_initial_file,
            quit_app,
            commands::open_file_dialog,
            commands::open_folder_dialog,
            commands::open_image_dialog,
            commands::read_file,
            commands::open_markdown_preview,
            commands::ensure_full_content,
            commands::save_file,
            commands::save_file_as_dialog,
            watcher::watch_file,
            watcher::unwatch_file,
            commands::list_dir,
            commands::read_binary_file,
            commands::save_image,
            settings::get_settings,
            settings::set_settings,
            settings::get_install_lang,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}