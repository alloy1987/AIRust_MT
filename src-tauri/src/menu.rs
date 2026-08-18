use tauri::menu::{
    CheckMenuItemBuilder, Menu, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder,
};
use tauri::{AppHandle, Emitter};

const MENU_NEW: &str = "m-new";
const MENU_OPEN: &str = "m-open";
const MENU_SAVE: &str = "m-save";
const MENU_SAVE_AS: &str = "m-save-as";
const MENU_UNDO: &str = "m-undo";
const MENU_REDO: &str = "m-redo";
const MENU_FIND: &str = "m-find";
const MENU_SELECT_ALL: &str = "m-select-all";
const MENU_SIDEBAR: &str = "m-toggle-sidebar";
const MENU_THEME: &str = "m-toggle-theme";
const MENU_QUIT: &str = "m-quit";
const MENU_ABOUT: &str = "m-about";
const MENU_LICENSE: &str = "m-license";

pub fn build(app: &AppHandle) -> tauri::Result<()> {
    // 不注册 accelerator：所有快捷键统一由前端 keydown 处理。
    // 若原生菜单也注册同一组合键，Windows 上一次按键会同时触发
    // 菜单事件与前端 keydown，导致「按一次 Ctrl+N 新建两个标签」这类双触发。
    let m_new = MenuItemBuilder::with_id(MENU_NEW, "新建").build(app)?;
    let m_open = MenuItemBuilder::with_id(MENU_OPEN, "打开…").build(app)?;
    let m_save = MenuItemBuilder::with_id(MENU_SAVE, "保存").build(app)?;
    let m_save_as = MenuItemBuilder::with_id(MENU_SAVE_AS, "另存为…").build(app)?;
    let m_undo = MenuItemBuilder::with_id(MENU_UNDO, "撤销").build(app)?;
    let m_redo = MenuItemBuilder::with_id(MENU_REDO, "重做").build(app)?;
    let m_find = MenuItemBuilder::with_id(MENU_FIND, "查找 / 替换").build(app)?;
    let m_select_all = MenuItemBuilder::with_id(MENU_SELECT_ALL, "全选").build(app)?;
    let m_sidebar = CheckMenuItemBuilder::with_id(MENU_SIDEBAR, "显示侧边栏")
        .checked(true)
        .build(app)?;
    let m_theme = MenuItemBuilder::with_id(MENU_THEME, "切换明暗主题").build(app)?;
    let m_about = MenuItemBuilder::with_id(MENU_ABOUT, "关于 AIRust_MT").build(app)?;
    let m_license = MenuItemBuilder::with_id(MENU_LICENSE, "许可证").build(app)?;
    let m_quit = MenuItemBuilder::with_id(MENU_QUIT, "退出").build(app)?;

    let sep = PredefinedMenuItem::separator(app)?;

    let file_menu = SubmenuBuilder::new(app, "文件(_F)")
        .items(&[&m_new, &m_open, &sep, &m_save, &m_save_as, &sep, &m_quit])
        .build()?;
    let edit_menu = SubmenuBuilder::new(app, "编辑(_E)")
        .items(&[&m_undo, &m_redo, &sep, &m_find, &m_select_all])
        .build()?;
    let view_menu = SubmenuBuilder::new(app, "视图(_V)")
        .items(&[&m_sidebar, &m_theme])
        .build()?;
    let help_menu = SubmenuBuilder::new(app, "帮助(_H)")
        .items(&[&m_about, &m_license])
        .build()?;

    let menu = Menu::with_items(app, &[&file_menu, &edit_menu, &view_menu, &help_menu])?;
    app.set_menu(menu).map(|_| ())
}

pub fn handle(app: &AppHandle, id: &str) {
    match id {
        MENU_QUIT => {
            // 不直接退出：通知前端走「检查未保存标签」的统一关闭流程
            let _ = app.emit("menu", "m-quit");
        }
        MENU_ABOUT => {
            let _ = app.emit("menu", "about");
        }
        MENU_LICENSE => {
            let _ = app.emit("menu", "license");
        }
        _ => {
            let _ = app.emit("menu", id.to_string());
        }
    }
}