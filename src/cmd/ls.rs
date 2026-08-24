use std::{
    error::Error,
    path::{Component, Path},
};

use colored::Colorize;
use tabled::{Table, Tabled};

use crate::archive;

use super::LsArgs;

#[derive(Tabled)]
struct FileDetails {
    name: String,
    size: u64,
    compressed_size: u64,
    last_modified: String,
}

impl FileDetails {
    fn from_node(node: &archive::ArchiveNode) -> Self {
        let last_modified = match &node.last_modified {
            Some(last_modified) => last_modified.to_owned(),
            _ => String::new(),
        };

        Self {
            name: node.name.clone(),
            size: node.size,
            compressed_size: node.compressed_size,
            last_modified,
        }
    }
}

fn list_file_details(node: &archive::ArchiveNode) {
    let mut all = vec![];
    let mut files = vec![];
    if node.is_dir {
        for (_, v) in &node.children {
            if v.is_dir {
                all.push(FileDetails::from_node(v));
                continue;
            }

            files.push(FileDetails::from_node(v));
        }

        all.append(&mut files);
    } else {
        all.push(FileDetails::from_node(node));
    }

    println!("{}", Table::new(all));
}

fn list_file(node: &archive::ArchiveNode, path: &str) {
    let p = Path::new(path);
    let components = p.components();
    let mut current_node = node;

    for c in components {
        if let Component::Normal(c) = c {
            let children = &current_node.children;
            let c = c.to_str().unwrap();

            for (k, v) in children {
                if k == c {
                    current_node = v;
                    continue;
                }
            }
        }
    }

    list_file_details(current_node);
}

pub fn list(options: LsArgs) -> Result<(), Box<dyn Error>> {
    let LsArgs {
        file,
        path,
        recursive,
    } = options;
    let (archive_node, path_existing_map) = archive::get_zip_structure(file, path)?;

    // println!("{:#?}", archive_node);

    if recursive {
    } else {
        if !path_existing_map.is_empty() {
            for (k, v) in path_existing_map {
                if !v {
                    println!("{k}: No such file or directory");
                } else {
                    let title = format!("{k}: ");
                    println!("{}", title.bold().green());
                    list_file(&archive_node, &k);
                }
            }
        } else {
            let mut files = vec![];
            for (_, v) in &archive_node.children {
                files.push(FileDetails::from_node(v));
            }

            println!("{}", Table::new(files));
        }
    }

    Ok(())
}
