use std::path::PathBuf;

use thiserror::Error;

use crate::{config, system};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("A note named \"{0}\" already exists.")]
    AlreadyExists(String),
    #[error("No note named \"{0}\" exists.")]
    NotFound(String),
    #[error(
        "A config file already exists {}.\
        \n To overwrite it with the default use `--force`.",
        .0.display()
    )]
    ConfigAlreadyExists(PathBuf),
    #[error(
        "A note named \"{0}\" already exists.\
        \n To overwrite it use `--force`."
    )]
    RenameAlreadyExists(String),
    #[error(
        "Can't restore note, because a note named \"{0}\" already exists.\
        \n Use `--new-name` to change the name of the restored note.\
        \n Use `--force` to replace the existing note.\
        \n Or remove/archive the existing note manually."
    )]
    RestoreAlreadyExists(String),
    #[error("Archiving failed, because file \"{0}\" already exists.")]
    ArchiveAlreadyExists(String),
    #[error("{0}")]
    SystemError(#[from] system::error::Error),
    #[error("{0}")]
    ConfigError(#[from] config::Error),
}
