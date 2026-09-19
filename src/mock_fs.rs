#![allow(clippy::unwrap_used)] // test-only
use std::path::Path;
use std::path::PathBuf;

use anyhow::{anyhow, Result};

use crate::file_operations::FileOperations;

fn extract_file_name(root_dir_path: &Path, file_path: &Path) -> Result<String> {
    if !file_path.starts_with(root_dir_path) {
        return Err(anyhow!("File not in rn root directory"));
    }
    let root_dir_path_len = root_dir_path.iter().count();
    let file_path_len = file_path.iter().count();
    if root_dir_path_len + 1 != file_path_len {
        return Err(anyhow!("Path does not point to file in rn root directory"));
    }
    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_owned();
    Ok(file_name)
}

#[allow(unused)]
pub struct MockFileSystem {
    opened_files: Vec<PathBuf>,
    rn_root_dir: PathBuf,
    files: Vec<String>,
}

#[allow(unused)]
impl MockFileSystem {
    pub fn new(rn_root_dir: PathBuf, notes: Vec<String>) -> Self {
        Self {
            opened_files: Vec::new(),
            rn_root_dir,
            files: notes,
        }
    }

    pub fn opened_files(&self) -> &Vec<PathBuf> {
        &self.opened_files
    }

    fn is_file(&self, path: &Path) -> bool {
        if let Ok(file_name) = extract_file_name(&self.rn_root_dir, path) {
            if self.files.contains(&file_name) {
                return true;
            }
        }
        false
    }

    fn is_dir(&self, path: &Path) -> bool {
        *path == self.rn_root_dir
    }
}

impl FileOperations for MockFileSystem {
    fn get_files(&self, dir: &Path) -> Result<Vec<String>> {
        if *dir == self.rn_root_dir {
            return Ok(self.files.clone());
        }
        Err(anyhow!("Directory does not exist"))
    }

    fn delete_file(&mut self, path: &Path) -> Result<()> {
        let file_name = extract_file_name(&self.rn_root_dir, path)?;
        let file_index = self.files.iter().position(|f| *f == file_name);
        if let Some(file_index) = file_index {
            let _ = self.files.remove(file_index);
            return Ok(());
        }
        Err(anyhow!("File does not exist"))
    }

    fn create_file(&mut self, path: &Path) -> Result<()> {
        extract_file_name(&self.rn_root_dir, path).map(|file_name| {
            self.files.push(file_name);
        })
    }

    fn create_dir(&mut self, path: &Path) -> Result<()> {
        if *path == self.rn_root_dir {
            return Err(anyhow!("{:?} already exists", self.rn_root_dir));
        }
        Err(anyhow!("can't create directories in mock fs"))
    }

    fn open_file(&mut self, _editor_command: &str, path: &Path) -> Result<()> {
        if !self.is_file(path) {
            return Err(anyhow!("Can't open because its not a file"));
        }
        self.opened_files.push(PathBuf::from(path));
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        self.is_dir(path) || self.is_file(path)
    }

    fn read_file(&self, _path: &Path) -> Result<String> {
        Err(anyhow!("Can't read from file in mock file system"))
    }

    fn write_file(&mut self, _path: &Path, _value: &str) -> Result<()> {
        Err(anyhow!("Can't write to file in mock file system"))
    }

    fn copy_file(&mut self, _source_path: &Path, _destination_path: &Path) -> Result<()> {
        Err(anyhow!("Can't copy files in mock file system"))
    }

    fn rename_file(&mut self, _source_path: &Path, _destination_path: &Path) -> Result<()> {
        Err(anyhow!("Can't rename files in mock file system"))
    }
}
