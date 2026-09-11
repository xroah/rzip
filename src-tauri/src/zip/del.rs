use std::{
    env,
    error::Error,
    fs::{self, File},
    io,
    path::PathBuf,
    time::SystemTime,
};

use serde::Deserialize;
use zip::{ZipArchive, ZipWriter};

#[derive(Deserialize)]
pub struct DelOptions {
    files: Vec<PathBuf>,
    target: PathBuf,
    password: Option<String>,
}

fn get_tmp_name() -> String {
    let t = SystemTime::now();
    let mills = t
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    format!(".tmp_{mills}")
}

pub fn delete(DelOptions { files, target, .. }: DelOptions) -> Result<(), Box<dyn Error>> {
    let zip_file = File::open(&target)?;
    let tmp_name = get_tmp_name();
    let tmp_file = env::home_dir().unwrap().join(".rzip").join(tmp_name);
    let mut zip_archive = ZipArchive::new(zip_file)?;
    let output = File::create(&tmp_file)?;
    let mut zip_writer = ZipWriter::new(output);

    'outer: for i in 0..zip_archive.len() {
        let mut f = zip_archive.by_index(i)?;
        let name = f.name();

        for to_del in &files {
            let to_del_str = to_del.to_str().unwrap().trim_end_matches("/");

            if name.starts_with(to_del_str)
                || name.starts_with("__MACOSX")
                || name.starts_with(".DS_Store")
            {
                continue 'outer;
            }
        }

        let options = f.options();

        if f.is_dir() {
            zip_writer.add_directory(name, options)?;
        } else {
            zip_writer.start_file(name, options)?;
            io::copy(&mut f, &mut zip_writer)?;
        }
    }

    zip_writer.finish()?;
    fs::copy(&tmp_file, target)?;

    let _ = fs::remove_file(tmp_file);

    Ok(())
}
