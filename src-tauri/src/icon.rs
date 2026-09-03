use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod file;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileIcon {
    name: String,
    exts: Vec<String>,
    file_names: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct Manifest {
    icon_map: BTreeMap<String, u16>,
    code_map: BTreeMap<u16, String>,
}

pub fn get_icon_manifest() -> Manifest {
    let ret = serde_json::from_str::<Vec<FileIcon>>(file::FILE);
    let mut manifest = Manifest {
        icon_map: BTreeMap::new(),
        code_map: BTreeMap::new(),
    };

    if let Ok(ret) = ret {
        for (idx, file_icon) in ret.iter().enumerate() {
            let code = idx as u16;

            manifest.code_map.insert(code, file_icon.name.clone());

            for ext in &file_icon.exts {
                manifest.icon_map.insert(ext.clone(), code);
            }

            if let Some(file_names) = &file_icon.file_names {
                for file_name in file_names {
                    manifest.icon_map.insert(file_name.clone(), code);
                }
            }
        }
    }

    manifest
}
