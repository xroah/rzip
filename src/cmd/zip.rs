use std::{
    env,
    error::Error,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use zip::{
    CompressionMethod, ZipWriter,
    write::{FileOptions, SimpleFileOptions},
};

use crate::error::ZipError;

fn zip_recursive<F>(file_path: &Path, cb: &mut F) -> Result<(), Box<dyn Error>>
where
    F: FnMut(&Path) -> Result<(), Box<dyn Error>>,
{
    cb(file_path)?;

    if file_path.is_dir() {
        let sub = fs::read_dir(file_path)?;

        for ret in sub {
            let e = ret?;

            zip_recursive(&e.path(), cb)?;
        }
    }

    Ok(())
}

fn get_default_file_name() -> PathBuf {
    let mut count = 1;

    loop {
        let name = format!("new_archive_{}.zip", count);
        let p = Path::new(&name);

        if p.exists() {
            count += 1;

            continue;
        }

        break p.to_path_buf();
    }
}

fn copy_file(
    src: &Path,
    name: &str,
    dst: &mut ZipWriter<File>,
    options: FileOptions<'_, ()>,
) -> Result<(), Box<dyn Error>> {
    dst.start_file(name, options)?;
    let mut f = File::open(src)?;

    io::copy(&mut f, dst)?;

    Ok(())
}

pub fn compress(
    super::Args {
        files,
        output,
        overwrite,
        dir,
        ..
    }: super::Args,
) -> Result<(), Box<dyn std::error::Error>> {
    if files.len() == 0 {
        return Ok(());
    }

    let current_dir = env::current_dir().unwrap();
    let file_name = if let Some(output) = output {
        output
    } else {
        get_default_file_name()
    };
    let dst_file = current_dir.join(&file_name);

    if dst_file.exists() {
        if overwrite {
            fs::remove_file(&dst_file)?;
        } else {
            return Err(Box::new(ZipError::FileExist(dst_file)));
        }
    }

    let mut zip_dst = ZipWriter::new(File::create(&dst_file)?);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0755);
    let strip_name: for<'a> fn(&'a str) -> &'a str =
        |name: &str| name.strip_prefix("/").unwrap_or(name);

    for f in &files {
        if f.exists() {
            if f.is_dir() {
                let normalized = f.canonicalize().unwrap();
                let base_path = if dir {
                    normalized.parent().unwrap().to_str().unwrap()
                } else {
                    normalized.to_str().unwrap()
                };

                zip_recursive(&normalized, &mut |p| {
                    let name = p.to_str().unwrap().strip_prefix(base_path).unwrap();
                    let name = strip_name(name);

                    if !name.is_empty() {
                        if p.is_dir() {
                            zip_dst.add_directory(format!("{}/", name), options)?;
                        } else {
                            copy_file(p, name, &mut zip_dst, options)?;
                        }
                    }

                    Ok(())
                })?;
            } else {
                let file_name = f.file_name().unwrap().to_str().unwrap();

                copy_file(&f, strip_name(file_name), &mut zip_dst, options)?;
            }
        } else {
            let e = ZipError::FileNotFound(f.to_path_buf());

            fs::remove_file(dst_file)?;

            return Err(Box::new(e));
        }
    }

    let ret = zip_dst.finish();

    if let Err(e) = ret {
        fs::remove_file(dst_file)?;

        return Err(Box::new(e));
    }

    Ok(())
}
