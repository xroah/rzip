use std::{
    collections::BTreeMap,
    error::Error,
    fs::File,
    path::{Component, Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use zip::{DateTime, ZipArchive};

trait PadLeadingZero {
    fn pad_leading_zero(self) -> String;
}

impl PadLeadingZero for u8 {
    fn pad_leading_zero(self) -> String {
        if self < 10 {
            format!("0{}", self)
        } else {
            self.to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveNode {
    pub name: String,
    pub is_dir: bool,
    pub last_modified: Option<String>,
    pub size: u64,
    pub compressed_size: u64,
    pub children: BTreeMap<String, ArchiveNode>,
}

pub fn get_zip_structure(file_path: PathBuf) -> Result<ArchiveNode, Box<dyn Error>> {
    let mut root = ArchiveNode {
        name: String::from("root"),
        is_dir: true,
        last_modified: None,
        size: 0,
        compressed_size: 0,
        children: BTreeMap::new(),
    };
    let file = File::open(file_path)?;
    let mut zip_file: ZipArchive<File> = ZipArchive::new(file)?;
    let len = zip_file.len();

    for i in 0..len {
        let f = zip_file.by_index(i)?;
        let n = f.name().trim_end_matches("/");
        let p = Path::new(n);

        let components: Vec<_> = p
            .components()
            .filter_map(|c| match c {
                Component::Normal(os) => os.to_str(),
                _ => None,
            })
            .collect();
        let mut current_node = &mut root;
        let len = components.len();
        let is_dir = f.is_dir();
        let size = f.size();
        let compressed_size = f.compressed_size();
        let last_modified = f.last_modified();

        for (idx, c) in components.iter().enumerate() {
            let is_last = idx == len - 1;
            let is_file = is_last && !is_dir;
            current_node = current_node
                .children
                .entry(c.to_string())
                .or_insert(ArchiveNode {
                    name: c.to_string(),
                    is_dir: !is_last || is_dir,
                    last_modified: if is_file {
                        format_date_time(last_modified)
                    } else {
                        None
                    },
                    size: if is_file { size } else { 0 },
                    compressed_size: if is_file { compressed_size } else { 0 },
                    children: BTreeMap::new(),
                });
        }
    }

    Ok(root)
}

fn format_date_time(dt: Option<DateTime>) -> Option<String> {
    let Some(dt) = dt else {
        return None;
    };

    Some(format!(
        "{}-{}-{} {}:{}:{}",
        dt.year(),
        dt.month().pad_leading_zero(),
        dt.day().pad_leading_zero(),
        dt.hour().pad_leading_zero(),
        dt.minute().pad_leading_zero(),
        dt.second().pad_leading_zero()
    ))
}

pub fn get_zip_json_structure(zip_file: PathBuf) -> Result<String, Box<dyn Error>>{
    let structure = get_zip_structure(zip_file)?;
    let ret = serde_json::to_string(&structure)?;

    Ok(ret)
}
