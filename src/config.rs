use std::path::PathBuf;

use anyhow::Error;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    error::{InternalError, SystemError},
    file_operations::FileOperations,
};

pub mod value_names {
    pub const EDITOR: &str = "editor";

    pub const ALL: [&str; 1] = [EDITOR];
}

fn config_dir() -> Result<PathBuf> {
    let mut path = std::env::home_dir().ok_or(SystemError::NoHomeDir)?;
    path.push(".config");
    Ok(path)
}

pub fn config_file() -> Result<PathBuf> {
    let mut path = config_dir()?;
    path.push("rn");
    path.push("rn.toml");
    Ok(path)
}

#[derive(Debug, Clone, Serialize)]
pub struct Config {
    pub editor: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            editor: String::from("nvim"),
        }
    }
}

impl Config {
    pub fn build<FS: FileOperations>(fs: &FS) -> Result<Self> {
        let mut config = Self::default();
        config.apply(PartialConfig::from_config_file(fs)?);
        Ok(config)
    }

    fn apply(&mut self, partial_config: PartialConfig) {
        if let Some(editor) = partial_config.editor {
            self.editor = editor;
        }
    }

    pub fn to_toml(&self) -> Result<String> {
        toml::to_string(self).map_err(|e| InternalError::<Error>(e.into()).into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PartialConfig {
    pub editor: Option<String>,
}

impl PartialConfig {
    pub fn from_config_file<FS: FileOperations>(fs: &FS) -> Result<Self> {
        let config_file_path = config_file()?;
        if let Ok(config_toml) = fs.read_file(&config_file_path) {
            return Ok(toml::from_str(&config_toml)?);
        }
        Ok(Self::default())
    }
}
