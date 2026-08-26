use std::{
    env,
    error::Error,
    fs::{self, File},
    path::PathBuf,
};

use colored::Colorize;
use zip::ZipArchive;

pub fn extract(
    super::UnzipArgs {
        file,
        output,
        password,
    }: super::UnzipArgs,
) -> Result<(), Box<dyn Error>> {
    let zip_file = File::open(file)?;
    let mut zip_archive = ZipArchive::new(zip_file)?;
    let output_path = if let Some(output) = output {
        output
    } else {
        PathBuf::from("./")
    };

    if !output_path.exists() {
        fs::create_dir_all(&output_path)?;
    }

    if let Some(pwd) = password {
        let pwd_bytes = pwd.bytes().collect::<Vec<u8>>();
    } else {
        zip_archive.extract(&output_path)?;
    }

    println!(
        "{}",
        format!(
            "Extracted to {}",
            env::current_dir()?
                .join(output_path)
                .canonicalize()?
                .to_str()
                .unwrap()
                .green()
        ).green()
    );

    Ok(())
}
