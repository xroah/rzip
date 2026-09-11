use std::{fs, path::PathBuf};

use tauri::{LogicalSize, Manager, Size};

mod icon;
mod js_api;
mod zip;
mod utils;

pub fn get_app_dir() -> PathBuf {
    home::home_dir().unwrap().join(".rzip")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_dir = get_app_dir();

    if !app_dir.exists() {
        fs::create_dir(app_dir).unwrap();
    }

    tauri::Builder::default()
        .setup(|app| {
            let Some(main_window) = app.get_webview_window("main") else {
                return Ok(());
            };

            main_window.set_min_size(Some(Size::Logical(LogicalSize {
                width: 800f64,
                height: 600f64,
            })))?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            js_api::get_zip_json,
            js_api::get_icon_manifest,
            js_api::is_filename_valid,
            js_api::del_zip_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
