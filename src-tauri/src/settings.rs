use serde_json::Value;
use tauri::{AppHandle, Manager};

fn settings_path(app: &AppHandle) -> std::path::PathBuf {
    app.path()
        .app_config_dir()
        .unwrap_or_default()
        .join("settings.json")
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<Value, String> {
    let path = settings_path(&app);
    match std::fs::read_to_string(&path) {
        Ok(text) => serde_json::from_str(&text).map_err(|e| format!("解析设置失败: {e}")),
        Err(_) => Ok(serde_json::json!({})),
    }
}

#[tauri::command]
pub fn set_settings(app: AppHandle, value: Value) -> Result<(), String> {
    let path = settings_path(&app);
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("创建配置目录失败: {e}"))?;
    }
    let text = serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?;
    std::fs::write(&path, text).map_err(|e| format!("写入设置失败: {e}"))
}

/// 安装器在安装时把用户选择的语言写入了注册表（见 `nsis/installer-hooks.nsh`）。
/// 应用「首次启动且尚未设置过语言」时读取它，让安装语言能映射到界面语言。
/// 非 Windows 平台返回 None。
#[tauri::command]
pub fn get_install_lang() -> Result<Option<String>, String> {
    #[cfg(windows)]
    {
        use std::io::ErrorKind;
        use winreg::enums::HKEY_CURRENT_USER;
        use winreg::RegKey;

        const REG_ROOT: &str = r"Software\AIRust_MT";
        const REG_VALUE: &str = "InstallLang";

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        match hkcu.open_subkey(REG_ROOT) {
            Ok(key) => match key.get_value::<String, _>(REG_VALUE) {
                Ok(v) if !v.is_empty() => Ok(Some(v)),
                _ => Ok(None),
            },
            // 从未安装过 / 未选语言：当作没有
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
            Err(e) => Err(format!("读取安装语言失败: {e}")),
        }
    }
    #[cfg(not(windows))]
    {
        Ok(None)
    }
}