use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fs::File,
    path::{Component, Path, PathBuf},
    rc::{Rc, Weak},
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

type RefCellNode = RefCell<ArchiveNode>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveNode {
    pub name: String,
    pub is_dir: bool,
    pub last_modified: String,
    pub size: u64,
    pub compressed_size: u64,
    pub children: HashMap<String, Rc<RefCellNode>>,
    #[serde(skip_serializing)]
    pub parent: Weak<RefCellNode>,
}

pub fn get_zip_structure(file_path: PathBuf) -> Result<Rc<RefCellNode>, Box<dyn Error>> {
    let root = Rc::new(RefCell::new(ArchiveNode {
        name: String::from("root"),
        is_dir: true,
        last_modified: String::new(),
        size: 0,
        compressed_size: 0,
        children: HashMap::new(),
        parent: Weak::new(),
    }));
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
        let mut current_node = root.clone();
        let is_dir = f.is_dir();
        let size = f.size();
        let compressed_size = f.compressed_size();
        let last_modified = f.last_modified();

        for c in components {
            let rc_node = Rc::new(RefCell::new(ArchiveNode {
                name: c.to_string(),
                is_dir: is_dir,
                last_modified: format_date_time(last_modified),
                size: if !is_dir { size } else { 0 },
                compressed_size: if !is_dir { compressed_size } else { 0 },
                children: HashMap::new(),
                parent: Rc::downgrade(&current_node),
            }));
            let children = current_node
                .borrow_mut()
                .children
                .entry(c.to_string())
                .or_insert(rc_node)
                .clone();

            let parent = children.borrow_mut().parent.upgrade();
            current_node = children;

            if let Some(p) = parent {
                let mut p = p.borrow_mut();
                p.size += size;
                p.compressed_size += compressed_size;
            }
        }
    }

    Ok(root)
}

fn format_date_time(dt: Option<DateTime>) -> String {
    let Some(dt) = dt else {
        return String::new();
    };

    format!(
        "{}-{}-{} {}:{}:{}",
        dt.year(),
        dt.month().pad_leading_zero(),
        dt.day().pad_leading_zero(),
        dt.hour().pad_leading_zero(),
        dt.minute().pad_leading_zero(),
        dt.second().pad_leading_zero()
    )
}

pub fn get_zip_json_structure(zip_file: PathBuf) -> Result<String, Box<dyn Error>> {
    let structure = get_zip_structure(zip_file)?;
    let ret = serde_json::to_string(&structure)?;

    Ok(ret)
}
