use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{Emitter, Manager};
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
async fn open_file(app: tauri::AppHandle) -> Result<(String, String, String), String> {
    let file_path = app
        .dialog()
        .file()
        .add_filter("Markdown", &["md", "markdown"])
        .blocking_pick_file()
        .ok_or("cancelled")?
        .into_path()
        .map_err(|e| e.to_string())?;

    let content = std::fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let name = file_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let path = file_path.to_string_lossy().into_owned();

    Ok((name, content, path))
}

#[tauri::command]
async fn save_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_file_as(app: tauri::AppHandle, content: String) -> Result<(String, String), String> {
    let file_path = app
        .dialog()
        .file()
        .add_filter("Markdown", &["md", "markdown"])
        .blocking_save_file()
        .ok_or("cancelled")?
        .into_path()
        .map_err(|e| e.to_string())?;

    std::fs::write(&file_path, &content).map_err(|e| e.to_string())?;

    let name = file_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let path = file_path.to_string_lossy().into_owned();

    Ok((name, path))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let open = MenuItem::with_id(app, "open", "開く", true, Some("CmdOrCtrl+O"))?;
            let save = MenuItem::with_id(app, "save", "保存", true, Some("CmdOrCtrl+S"))?;
            let save_as = MenuItem::with_id(
                app,
                "save_as",
                "別名で保存",
                true,
                Some("CmdOrCtrl+Shift+S"),
            )?;

            #[cfg(target_os = "macos")]
            let file_menu =
                { Submenu::with_items(app, "ファイル", true, &[&open, &save, &save_as])? };

            #[cfg(not(target_os = "macos"))]
            let file_menu = {
                let sep = PredefinedMenuItem::separator(app)?;
                let quit = PredefinedMenuItem::quit(app, Some("終了"))?;
                Submenu::with_items(
                    app,
                    "ファイル",
                    true,
                    &[&open, &save, &save_as, &sep, &quit],
                )?
            };

            #[cfg(target_os = "macos")]
            let menu = {
                let app_menu = Submenu::with_items(
                    app,
                    app.config().product_name.as_deref().unwrap_or("md-blog"),
                    true,
                    &[
                        &PredefinedMenuItem::about(app, None, None)?,
                        &PredefinedMenuItem::separator(app)?,
                        &PredefinedMenuItem::services(app, None)?,
                        &PredefinedMenuItem::separator(app)?,
                        &PredefinedMenuItem::hide(app, None)?,
                        &PredefinedMenuItem::hide_others(app, None)?,
                        &PredefinedMenuItem::show_all(app, None)?,
                        &PredefinedMenuItem::separator(app)?,
                        &PredefinedMenuItem::quit(app, Some("終了"))?,
                    ],
                )?;
                Menu::with_items(app, &[&app_menu, &file_menu])?
            };

            #[cfg(not(target_os = "macos"))]
            let menu = Menu::with_items(app, &[&file_menu])?;

            app.set_menu(menu)?;

            app.on_menu_event(|app, event| {
                let window = match app.get_webview_window("main") {
                    Some(w) => w,
                    None => return,
                };
                match event.id().as_ref() {
                    "open" => {
                        let _ = window.emit("menu:open", ());
                    }
                    "save" => {
                        let _ = window.emit("menu:save", ());
                    }
                    "save_as" => {
                        let _ = window.emit("menu:save-as", ());
                    }
                    _ => {}
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_file, save_file, save_file_as])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
