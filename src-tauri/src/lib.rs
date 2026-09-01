use std::{fs, path::PathBuf};

pub mod zip;

#[tauri::command]
fn get_json(file: String) -> Result<String, String> {
    let file_path = PathBuf::from(file);
    let ret = zip::archive::get_zip_json_structure(file_path);

    match ret {
        Ok(json) => Ok(json),
        Err(e) => Err(e.to_string()),
    }
}

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
        .invoke_handler(tauri::generate_handler![get_json])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
