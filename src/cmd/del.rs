use std::{
    env,
    error::Error,
    fs::{self, File},
    io,
};

use zip::{ZipArchive, ZipWriter};

pub fn delete(
    super::DelOrAddArgs {
        files,
        target,
        password,
    }: super::DelOrAddArgs,
) -> Result<(), Box<dyn Error>> {
    let zip_file = File::open(&target)?;
    let tmp_name = format!(".tmp_{}", target.file_name().unwrap().to_str().unwrap());
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

    if let Err(err) = zip_writer.finish() {
        println!("{err:?}");
    } else {
        fs::copy(&tmp_file, target)?;
    }

    fs::remove_file(tmp_file)?;

    Ok(())
}
