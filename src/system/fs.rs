use std::fs::File;
use std::{fs, path::Path};

use crate::system::error::Error;
use crate::system::error::Result;

/// Checks whether an object exists behind given `path`.
///
/// Returns `Some(true)` if an object exists, `Some(false)` if not and `Err(_)` if existence can
/// neither be confirmed or denied.
///
/// # Arguments
///
/// * `path` - path to check existence for
pub fn exists(path: &Path) -> Result<bool> {
    path.try_exists().map_err(Into::into)
}

pub fn create_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn list_files(dir: &Path) -> Result<Vec<String>> {
    let mut files = Vec::new();
    let dir = fs::read_dir(dir)?;
    for entry in dir {
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

pub fn is_file(path: &Path) -> bool {
    path.is_file()
}

pub fn delete_file(path: &Path) -> Result<()> {
    fs::remove_file(path)?;
    Ok(())
}

pub fn create_file(path: &Path) -> Result<()> {
    let _ = File::create_new(path)?;
    Ok(())
}

pub fn read_file(path: &Path) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if exists(path)? && !is_file(path) {
        return Err(Error::NotAFile(path.to_path_buf()).into());
    }
    fs::write(path, content).map_err(Into::into)
}

pub fn copy_file(source_path: &Path, destination_path: &Path) -> Result<()> {
    if !is_file(source_path) {
        return Err(Error::NotAFile(source_path.to_path_buf()).into());
    }
    fs::copy(source_path, destination_path)
        .map(|_| ())
        .map_err(Into::into)
}

pub fn rename_file(source_path: &Path, destination_path: &Path) -> Result<()> {
    if !is_file(source_path) {
        return Err(Error::NotAFile(source_path.to_path_buf()));
    }
    fs::rename(source_path, destination_path).map_err(Into::into)
}
