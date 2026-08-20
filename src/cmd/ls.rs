use std::{error::Error, path::PathBuf};

fn list_zip(file: &PathBuf, recursive: bool) -> Result<(), Box<dyn Error>> {
    let archive_node = crate::archive::get_zip_structure(file.to_path_buf())?;

    println!("{:#?}", archive_node);

    if !recursive {
        for (_, v) in &archive_node.children {
            println!("{}", v.name);
        }
    }

    Ok(())
}

pub fn list(files: Vec<PathBuf>, recursive: bool) {
    for f in files {
        let ret = list_zip(&f, recursive);

        if let Err(e) = ret {
            println!("{}: {}", f.to_str().unwrap(), e)
        }
    }
}
