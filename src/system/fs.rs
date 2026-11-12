use std::fs::File;
use std::{fs, path::Path};

use crate::system::error::Error;
use crate::system::error::Result;

/// Checks whether an object exists behind given `path`.
///
/// Returns `Some(true)` if an object exists, `Some(false)` if not and `Err(_)` if existence can
/// neither be confirmed or denied.
///
/// # Errors
///
/// Forwards errors from [`std::path::Path::try_exists`].
pub fn exists(path: &Path) -> Result<bool> {
    path.try_exists().map_err(Into::into)
}

/// Recursively create a directory and all of its parent components if they
/// are missing.
///
/// # Errors
///
/// Forwards errors from [`std::fs::create_dir_all`].
pub fn create_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path).map_err(Into::into)
}

/// Returns a list of all file names from given directory.
///
/// Ignores files if their name starts with a dot.
///
/// # Errors
///
/// * Forwards errors from [`std::fs::read_dir`] or other issues regarding e.g. permissions.
/// * If a file name could not be converted to a `String`, because it's not valid UTF-8.
pub fn list_files(dir_path: &Path) -> Result<Vec<String>> {
    let mut files = Vec::new();
    let dir_entries = fs::read_dir(dir_path)?;
    for entry in dir_entries {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let file_name = entry
            .file_name()
            .into_string()
            .map_err(Error::FileNameNoUTF8)?;
        if !file_name.starts_with('.') {
            files.push(file_name);
        }
    }
    Ok(files)
}

/// Returns `true` if the path exists on disk and is pointing at a regular file.
///
/// If you cannot access the metadata of the file, e.g. because of a
/// permission error, this will return `false`.
///
/// For further information see [`std::path::Path::is_file`].
#[must_use]
pub fn is_file(path: &Path) -> bool {
    path.is_file()
}

/// Delete the file behind given `path` from the filesystem.
///
/// # Errors
///
/// Forwards errors from [`std::fs::remove_file`].
pub fn delete_file(path: &Path) -> Result<()> {
    fs::remove_file(path).map_err(Into::into)
}

/// Creates a new file at given `path`.
///
/// # Errors
///
/// Forwards errors from [`std::fs::File::create_new`].
pub fn create_file(path: &Path) -> Result<()> {
    File::create_new(path).map(|_| ()).map_err(Into::into)
}

/// Read file behind given `path` to a `String`.
///
/// # Errors
///
/// Forwards errors from [`std::fs::read_to_string`].
pub fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path).map_err(Into::into)
}

/// Write `content` to a file, either overwriting an existing file or creating a new one.
///
/// # Errors
///
/// * There exists an object behind the given `path`, but its not a file.
/// * Forwards errors from [`fs::write`].
pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if exists(path)? && !is_file(path) {
        return Err(Error::NotAFile(path.to_path_buf()));
    }
    fs::write(path, content).map_err(Into::into)
}

/// Copy file from `source` to `destination`.
///
/// # Errors
///
/// * If `source` does not point to a file.
/// * Forwards errors from [`fs::copy`].
pub fn copy_file(source: &Path, destination: &Path) -> Result<()> {
    // TODO maybe should return an error, if `destination` already exists
    if !is_file(source) {
        return Err(Error::NotAFile(source.to_path_buf()));
    }
    fs::copy(source, destination)
        .map(|_| ())
        .map_err(Into::into)
}

/// Moves file from `source` to `destination`.
///
/// # Errors
///
/// * If `source` does not point to a file.
/// * Forwards errors from [`fs::rename`].
pub fn move_file(source: &Path, destination: &Path) -> Result<()> {
    // TODO maybe should return an error, if `destination` already exists
    if !is_file(source) {
        return Err(Error::NotAFile(source.to_path_buf()));
    }
    fs::rename(source, destination).map_err(Into::into)
}
