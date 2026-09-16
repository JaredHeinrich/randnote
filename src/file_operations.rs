use anyhow::Result;
use std::process::Stdio;
use std::{
    fs::{self, File},
    path::Path,
    process::Command,
};

use crate::error::FileSystemError;
use crate::error::SystemError;

fn check_command(command_name: &str) -> Result<()> {
    let check_command = "which";
    let status = std::process::Command::new(check_command)
        .arg(command_name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|_| SystemError::CommandNotInstalled(check_command.to_owned()))?
        .code()
        .unwrap_or(1);
    if status != 0 {
        Err(SystemError::CommandNotInstalled(command_name.to_owned()))?;
    }
    Ok(())
}

pub trait FileOperations {
    fn get_files(&self, dir: &Path) -> Result<Vec<String>>;
    fn delete_file(&mut self, path: &Path) -> Result<()>;
    fn create_file(&mut self, path: &Path) -> Result<()>;
    fn create_dir(&mut self, path: &Path) -> Result<()>;
    fn open_file(&mut self, editor_command: &str, path: &Path) -> Result<()>;
    fn exists(&self, path: &Path) -> bool;
    fn read_file(&self, path: &Path) -> Result<String>;
    fn write_file(&mut self, path: &Path, value: &str) -> Result<()>;
    fn copy_file(&mut self, source_path: &Path, destination_path: &Path) -> Result<()>;
    fn rename_file(&mut self, source_path: &Path, destination_path: &Path) -> Result<()>;
}

pub struct FileSystem;
impl FileOperations for FileSystem {
    fn get_files(&self, dir: &Path) -> Result<Vec<String>> {
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
                .map_err(FileSystemError::FileNameNoUTF8)?;
            if !file_name.starts_with('.') {
                files.push(file_name);
            }
        }
        Ok(files)
    }

    fn delete_file(&mut self, path: &Path) -> Result<()> {
        fs::remove_file(path).map_err(Into::into)
    }

    fn create_file(&mut self, path: &Path) -> Result<()> {
        File::create_new(path).map_err(Into::into).map(|_| ())
    }

    fn create_dir(&mut self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).map_err(Into::into)
    }

    fn open_file(&mut self, editor_command: &str, path: &Path) -> Result<()> {
        check_command(editor_command)?;
        if !path.is_file() {
            return Err(FileSystemError::NotAFile(path.to_path_buf()).into());
        }
        Command::new(editor_command)
            .arg(path.as_os_str())
            .status()
            .map(|_| ())
            .map_err(Into::into)
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn read_file(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path).map_err(Into::into)
    }

    fn write_file(&mut self, path: &Path, content: &str) -> Result<()> {
        if self.exists(path) && !path.is_file() {
            return Err(FileSystemError::NotAFile(path.to_path_buf()).into());
        }
        fs::write(path, content).map_err(Into::into)
    }

    fn copy_file(&mut self, source_path: &Path, destination_path: &Path) -> Result<()> {
        if !source_path.is_file() {
            return Err(FileSystemError::NotAFile(source_path.to_path_buf()).into());
        }
        fs::copy(source_path, destination_path)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn rename_file(&mut self, source_path: &Path, destination_path: &Path) -> Result<()> {
        if !source_path.is_file() {
            return Err(FileSystemError::NotAFile(source_path.to_path_buf()).into());
        }
        fs::rename(source_path, destination_path).map_err(Into::into)
    }
}
