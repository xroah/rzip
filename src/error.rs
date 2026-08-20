use std::{error::Error, fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum ZipError {
    FileNotFound(PathBuf),
}

impl Display for ZipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound(file_path) => {
                write!(f, "{:?} can not be found", file_path)
            }
        }
    }
}

impl Error for ZipError {
}