use std::{ffi::OsString, fmt::Display, path::PathBuf};

use thiserror::Error;

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
            Self::AlreadyExists(name) => writeln!(f, "A note named \"{name}\" already exists."),
            Self::NotFound(name) => writeln!(f, "No note named \"{name}\" exists."),
            Self::ConfigAlreadyExists(path) => {
                writeln!(f, "A config file already exists {}.", path.display())?;
                writeln!(f, "To overwrite it with the default use `--force`.")
            }
            Self::RenameAlreadyExists(name) => {
                writeln!(f, "A note named \"{name}\" already exists.")?;
                writeln!(f, "To overwrite it use `--force`.")
            }
            Self::RestoreAlreadyExists(name) => {
                writeln!(
                    f,
                    "Can't restore note, because a note named \"{name}\" already exists."
                )?;
                writeln!(
                    f,
                    "Use `--new-name` to change the name of the restored note."
                )?;
                writeln!(f, "Or remove/archive the existing note.")
            }
            Self::ArchiveAlreadyExists(name) => writeln!(
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
                writeln!(f, "The command \"{command}\" is not installed.")
            }
            Self::NoHomeDir => writeln!(f, "No home directory could be found."),
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
            Self::NotAFile(path) => writeln!(f, "\"{}\" is not a file.", path.display()),
            #[allow(clippy::unnecessary_debug_formatting)]
            Self::FileNameNoUTF8(file_name) => {
                writeln!(f, "File name {file_name:?} is no valid UTF-8.")
            }
            #[allow(clippy::unnecessary_debug_formatting)]
            Self::PathNoUTF8(path) => {
                writeln!(f, "Path {path:?} is no valid UTF-8.")
            }
            Self::NoParentDirectory => writeln!(f, "File has no parent directory."),
        }
    }
}

#[derive(Error, Debug)]
pub struct InternalError<E>(pub E);

impl<T: Display> Display for InternalError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Internal Error, you may open an issue on Github: \n {}",
            self.0
        )
    }
}
