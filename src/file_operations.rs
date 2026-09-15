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
    fn exists(&self, path: &Path) -> Result<bool>;
    fn read_file(&self, path: &Path) -> Result<String>;
    fn write_file(&mut self, path: &Path, value: &str) -> Result<()>;
    fn copy(&mut self, source_path: &Path, destination_path: &Path) -> Result<()>;
    fn rename(&mut self, source_path: &Path, destination_path: &Path) -> Result<()>;
}

pub struct FileSystem;
impl FileOperations for FileSystem {
    fn get_files(&self, dir: &Path) -> Result<Vec<String>> {
        let mut files = Vec::new();
        let dir = fs::read_dir(dir)?;
        for entry in dir {
            let entry = entry?;
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

    fn exists(&self, path: &Path) -> Result<bool> {
        fs::exists(path).map_err(Into::into)
    }

    fn read_file(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path).map_err(Into::into)
    }

    fn write_file(&mut self, file_path: &Path, content: &str) -> Result<()> {
        fs::write(file_path, content).map_err(Into::into)
    }

    fn copy(&mut self, source_path: &Path, destination_path: &Path) -> Result<()> {
        fs::copy(source_path, destination_path)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn rename(&mut self, source_path: &Path, destination_path: &Path) -> Result<()> {
        fs::rename(source_path, destination_path).map_err(Into::into)
    }
}
