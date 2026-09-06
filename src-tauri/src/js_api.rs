use std::path::PathBuf;

use crate::{icon, zip};

#[tauri::command]
pub fn get_zip_json(file: String) -> Result<String, String> {
    let file_path = PathBuf::from(file);
    let ret = zip::archive::get_json(file_path);

    match ret {
        Ok(json) => Ok(json),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_icon_manifest() -> icon::Manifest {
    icon::get_icon_manifest()
}
