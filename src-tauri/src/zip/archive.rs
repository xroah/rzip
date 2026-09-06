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

impl Default for ArchiveNode {
    fn default() -> Self {
        Self {
            name: Default::default(),
            is_dir: true,
            last_modified: Default::default(),
            size: Default::default(),
            compressed_size: Default::default(),
            children: HashMap::new(),
            parent: Default::default(),
            ext: Default::default(),
        }
    }
}

impl ArchiveNode {
    fn from_name(name: &str) -> Self {
        let mut ret = Self::default();
        ret.name = String::from(name);

        ret
    }
}

type RefCellNode = RefCell<ArchiveNode>;

#[derive(Debug, Serialize, Deserialize)]
struct ArchiveNode {
    name: String,
    is_dir: bool,
    last_modified: Option<String>,
    size: u64,
    compressed_size: u64,
    children: HashMap<String, Rc<RefCellNode>>,
    #[serde(skip_serializing)]
    parent: Weak<RefCellNode>,
    ext: Option<String>,
}

fn get_zip_structure(file_path: PathBuf) -> Result<Rc<RefCellNode>, Box<dyn Error>> {
    let root = Rc::new(RefCell::new(ArchiveNode::from_name("root")));
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
            let mut node = ArchiveNode::from_name(c);
            node.is_dir = is_dir;
            node.last_modified = format_date_time(last_modified);

            if !is_dir {
                node.size = size;
                node.compressed_size = compressed_size;
                node.ext = get_file_ext(p);
            }

            node.parent = Rc::downgrade(&current_node);
            let rc_node = Rc::new(RefCell::new(node));
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

fn get_file_ext(path: &Path) -> Option<String> {
    let Some(ext) = path.extension() else {
        return None;
    };
    let Some(ext) = ext.to_str() else {
        return None;
    };

    Some(String::from(ext))
}

pub fn get_json(zip_file: PathBuf) -> Result<String, Box<dyn Error>> {
    let structure = get_zip_structure(zip_file)?;
    let ret = serde_json::to_string(&structure)?;

    Ok(ret)
}
