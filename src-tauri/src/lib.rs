use std::{fs, path::PathBuf};

mod icon;
mod js_api;
mod zip;

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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            js_api::get_zip_json,
            js_api::get_icon_manifest
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
