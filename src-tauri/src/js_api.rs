use std::path::PathBuf;

use crate::{icon, utils, zip::{archive, del::DelOptions, del::delete}};

#[tauri::command]
pub fn get_zip_json(file: String) -> Result<String, String> {
    let file_path = PathBuf::from(file);
    let ret = archive::get_json(file_path);

    match ret {
        Ok(json) => Ok(json),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_icon_manifest() -> icon::Manifest {
    icon::get_icon_manifest()
}

#[tauri::command]
pub fn is_filename_valid(filename: &str) -> bool {
    utils::is_valid_mac_filename(filename)
}

#[tauri::command]
pub fn del_zip_files(options: DelOptions) -> String {
    let ret = delete(options);

    match ret {
        Ok(()) => String::new(),
        Err(e) => e.to_string()
    }
}