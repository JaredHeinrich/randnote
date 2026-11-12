use std::{ffi::OsString, io, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io Error: {0}")]
    IoError(#[from] io::Error),
    #[error("\"{}\" is not a file.", .0.display())]
    NotAFile(PathBuf),
    #[error("File name {0:?} is no valid UTF-8.")]
    FileNameNoUTF8(OsString),
    #[error("Path {0:?} is no valid UTF-8.")]
    PathNoUTF8(PathBuf),
    #[error("File has no parent directory.")]
    NoParentDirectory,
    #[error("Could not get home directory.")]
    NoHomeDir,
    #[error("The command \"{0}\" is not installed.")]
    CommandNotInstalled(String),
}

pub type Result<T> = std::result::Result<T, Error>;
