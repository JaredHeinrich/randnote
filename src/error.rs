use std::{ffi::OsString, fmt::Display, path::PathBuf};

use anyhow::{Error as AnyError};
use thiserror::Error;


#[derive(Error, Debug)]
pub struct ErrorContext(Vec<AnyError>);
impl Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in self.0.iter() {
            writeln!(f, "{e}")?;
        }
        Ok(())
    }
}
impl ErrorContext {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn add_error(&mut self, e: AnyError) {
        self.0.push(e);
    }
    pub fn contains_error(&self) -> bool {
        !self.0.is_empty()
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    AlreadyExists(String),
    NotFound(String),
    ConfigAlreadyExists(PathBuf),
    RenameAlreadyExists(String),
    RestoreAlreadyExists(String),
    ArchiveAlreadyExists(String),
}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists(name) => write!(f, "A note named \"{name}\" already exists."),
            Self::NotFound(name) => write!(f, "No note named \"{name}\" exists."),
            Self::ConfigAlreadyExists(path) => {
                write!(
                    f,
                    "A config file already exists {}.\n\
                    To overwrite it with the default use `--force`.",
                    path.display()
                )
            }
            Self::RenameAlreadyExists(name) => {
                write!(
                    f,
                    "A note named \"{name}\" already exists.\n\
                    To overwrite it use `--force`."
                )
            }
            Self::RestoreAlreadyExists(name) => {
                write!(
                    f,
                    "Can't restore note, because a note named \"{name}\" already exists.\n\
                    Use `--new-name` to change the name of the restored note.\n\
                    Use `--force` to replace the existing note.\n\
                    Or remove/archive the existing note manually."
                )
            }
            Self::ArchiveAlreadyExists(name) => write!(
                f,
                "Archiving failed, because file \"{name}\" already exists."
            ),
        }
    }
}

#[derive(Error, Debug)]
pub enum SystemError {
    CommandNotInstalled(String),
    NoHomeDir,
}

impl Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandNotInstalled(command) => {
                write!(f, "The command \"{command}\" is not installed.")
            }
            Self::NoHomeDir => write!(f, "No home directory could be found."),
        }
    }
}

#[derive(Error, Debug)]
pub enum FileSystemError {
    NotAFile(PathBuf),
    FileNameNoUTF8(OsString),
    PathNoUTF8(PathBuf),
    NoParentDirectory,
}

impl Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotAFile(path) => write!(f, "\"{}\" is not a file.", path.display()),
            #[allow(clippy::unnecessary_debug_formatting)]
            Self::FileNameNoUTF8(file_name) => {
                write!(f, "File name {file_name:?} is no valid UTF-8.")
            }
            #[allow(clippy::unnecessary_debug_formatting)]
            Self::PathNoUTF8(path) => {
                write!(f, "Path {path:?} is no valid UTF-8.")
            }
            Self::NoParentDirectory => write!(f, "File has no parent directory."),
        }
    }
}

#[derive(Error, Debug)]
pub struct InternalError<E>(pub E);

impl<T: Display> Display for InternalError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Internal Error, you may open an issue on Github: \n {}",
            self.0
        )
    }
}
