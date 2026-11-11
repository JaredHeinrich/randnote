use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::system;
use crate::system::env;
use crate::system::fs;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error when parsing config file:\n{0}")]
    ParsingError(#[from] toml::de::Error),
    #[error("Error when serializing config file:\n{0}")]
    SerializationError(#[from] toml::ser::Error),
    #[error("{0}")]
    SystemError(#[from] system::error::Error),
}

type Result<T> = std::result::Result<T, Error>;

fn config_dir() -> Result<PathBuf> {
    let mut path = env::home_dir()?;
    path.push(".config");
    Ok(path)
}

pub fn config_file() -> Result<PathBuf> {
    let mut path = config_dir()?;
    path.push("rn");
    path.push("rn.toml");
    Ok(path)
}

#[derive(Debug, Clone)]
pub struct Config {
    pub root_dir: PathBuf,
    pub editor: String,
}

impl Config {
    pub fn build() -> Result<Self> {
        let mut root_dir = env::home_dir()?;
        root_dir.push(".rn");
        let mut config = Self {
            root_dir,
            editor: String::new(),
        };
        config.apply(PartialConfig::default());
        config.apply(PartialConfig::from_config_file()?);
        Ok(config)
    }

    fn apply(&mut self, partial_config: PartialConfig) {
        if let Some(editor) = partial_config.editor {
            self.editor = editor;
        }
    }
}

pub mod value_names {
    pub const EDITOR: &str = "editor";

    pub const ALL: [&str; 1] = [EDITOR];
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialConfig {
    pub editor: Option<String>,
}

impl Default for PartialConfig {
    fn default() -> Self {
        Self {
            editor: Some("nvim".to_string()),
        }
    }
}

impl PartialConfig {
    pub fn empty() -> Self {
        Self { editor: None }
    }

    pub fn from_config_file() -> Result<Self> {
        let config_file = config_file()?;
        if let Ok(config_toml) = fs::read_file(&config_file) {
            return Ok(toml::from_str(&config_toml)?);
        }
        Ok(Self::empty())
    }

    pub fn to_toml(&self) -> Result<String> {
        toml::to_string(self).map_err(Into::into)
    }

    pub fn get_values_from_file<T: AsRef<str>>(value_names: &[T]) -> Result<Vec<(String, String)>> {
        let mut config_values: Vec<(String, String)> = Vec::new();
        let config = Self::from_config_file()?;
        for value_name in value_names {
            let value_name = value_name.as_ref();
            let value = match value_name {
                value_names::EDITOR => config.editor.as_ref(),
                _ => continue,
            };
            if let Some(value) = value {
                config_values.push((value_name.to_owned(), value.to_owned()));
            }
        }
        Ok(config_values)
    }
}
