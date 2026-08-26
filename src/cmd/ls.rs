use std::{
    error::Error,
    path::{Component, Path},
};

use colored::Colorize;
use tabled::{Table, Tabled};

use crate::archive::{ArchiveNode, get_zip_structure};

use super::LsArgs;

#[derive(Tabled)]
struct FileDetails {
    name: String,
    size: u64,
    compressed_size: u64,
    last_modified: String,
}

impl FileDetails {
    fn from_node(node: &ArchiveNode) -> Self {
        let last_modified = match &node.last_modified {
            Some(last_modified) => last_modified.to_owned(),
            _ => String::new(),
        };
        let name = if node.is_dir {
            node.name.blue().bold().to_string()
        } else {
            node.name.clone()
        };

        Self {
            name,
            size: node.size,
            compressed_size: node.compressed_size,
            last_modified,
        }
    }
}

fn list_file_details(node: &ArchiveNode) {
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

fn list_file(node: &ArchiveNode, path: &str) {
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

fn do_list_recursive(node: &ArchiveNode, space_num: usize) {
    let space_step: usize = 3;
    let children = &node.children;
    let mut dirs = vec![];
    let mut files = vec![];

    for (_, n) in children {
        if n.is_dir {
            dirs.push(n);
        } else {
            files.push(n);
        }
    }

    let hierarchy = space_num / space_step;
    let hierarchy = if hierarchy > 0 { hierarchy - 1 } else { 0 };
    let spaces = format!(
        "{}{}",
        "|  ".repeat(hierarchy).green(),
        if space_num > 0 { "|--" } else { "" }.green().bold()
    );

    for d in dirs {
        println!("{}{}", spaces, d.name.blue());

        do_list_recursive(d, space_num + space_step);
    }

    for f in files {
        println!("{}{}", spaces, f.name);
    }
}

fn list_recursive(node: &ArchiveNode, path: Option<&str>) {
    let mut current_node = node;

    if let Some(path) = path {
        let p = Path::new(path);
        let components = p.components();

        for c in components {
            for (_, child_node) in &current_node.children {
                if c.as_os_str().to_str().unwrap() == child_node.name {
                    current_node = child_node;
                    continue;
                }
            }
        }
    }

    do_list_recursive(current_node, 0);
}

pub fn list(
    LsArgs {
        file,
        path,
        recursive,
    }: LsArgs,
) -> Result<(), Box<dyn Error>> {
    let (archive_node, path_existing_map) = get_zip_structure(file, path)?;

    if !path_existing_map.is_empty() {
        let existing_count = path_existing_map.iter().filter(|&(_, v)| *v).count();
        let mut c: u8 = 0;

        for (k, v) in path_existing_map {
            if !v {
                println!("{k}: No such file or directory");
            } else {
                if existing_count > 1 {
                    let title = format!("{k}: ");

                    if c > 0 {
                        println!("\n");
                    }

                    println!("{}", title.bold().green());

                    c += 1;
                }

                if recursive {
                    list_recursive(&archive_node, Some(&k));
                } else {
                    list_file(&archive_node, &k);
                }
            }
        }
    } else {
        if !recursive {
            let mut files = vec![];
            let mut all = vec![];

            for (_, v) in &archive_node.children {
                if v.is_dir {
                    all.push(FileDetails::from_node(v));
                } else {
                    files.push(FileDetails::from_node(v));
                }
            }

            all.append(&mut files);

            println!("{}", Table::new(all));
        } else {
            list_recursive(&archive_node, None);
        }
    }

    Ok(())
}
