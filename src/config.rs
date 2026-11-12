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

/// Gets the path of the config directory `$HOME/.config`
///
/// # Errors
///
/// Return an error, if there is no home directory.
fn config_dir() -> Result<PathBuf> {
    let mut path = env::home_dir()?;
    path.push(".config");
    Ok(path)
}

/// Get the path of config file `$HOME/.config/rn/rn.toml`
///
/// # Errors
///
/// Return an error, if there is no home directory.
pub fn config_file() -> Result<PathBuf> {
    let mut path = config_dir()?;
    path.push("rn");
    path.push("rn.toml");
    Ok(path)
}

/// Configuration for the application
///
/// * `root_dir` - the root directory to store all application data, by default set to `$HOME/.rn`.
///   (currently not configurable by the user => will not be serialized to the config file)
/// * `editor` - editor used to open files
#[derive(Serialize, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub root_dir: PathBuf,
    pub editor: String,
}

impl Config {
    /// Create the config with default values.
    ///
    /// # Errors
    ///
    /// Returns an error if there is no home directory.
    #[allow(clippy::should_implement_trait)] // Intended to have `default` with `Result`
    pub fn default() -> Result<Self> {
        let mut root_dir = env::home_dir()?;
        root_dir.push(".rn");
        Ok(Self {
            root_dir,
            editor: "nvim".to_string(),
        })
    }

    /// Build the config by starting with default and then applying higher levels.
    ///
    /// # Errors
    ///
    /// * There is no home directory
    /// * There was an issues when parsing the config file
    pub fn build() -> Result<Self> {
        let mut config = Self::default()?;
        config.apply(PartialConfig::from_config_file()?);
        Ok(config)
    }

    /// Apply a `PartialConfig` to the given config.
    ///
    /// For each entry if `partial_config` a value replace the value of `config`.
    fn apply(&mut self, partial_config: PartialConfig) {
        if let Some(editor) = partial_config.editor {
            self.editor = editor;
        }
    }

    /// Serializes the config to toml format.
    ///
    /// # Errors
    ///
    /// Forwards errors from [`toml::to_string`].
    pub fn to_toml(&self) -> Result<String> {
        toml::to_string(self).map_err(Into::into)
    }
}

/// Possible value names in the configuration file.
pub mod value_names {
    pub const EDITOR: &str = "editor";

    pub const ALL: [&str; 1] = [EDITOR];
}

/// Partial configuration.
///
/// Used to e.g. load config from file where some fields may be empty.
#[derive(Deserialize, Debug, Clone)]
pub struct PartialConfig {
    pub editor: Option<String>,
}

impl PartialConfig {
    /// Create a partial config with all fields `None`.
    #[must_use]
    pub fn empty() -> Self {
        Self { editor: None }
    }

    /// Load partial config from config file.
    ///
    /// # Errors
    ///
    /// * There is no home directory
    /// * There was an issues when parsing the config file
    pub fn from_config_file() -> Result<Self> {
        let config_file = config_file()?;
        if let Ok(config_toml) = fs::read_file(&config_file) {
            return Ok(toml::from_str(&config_toml)?);
        }
        Ok(Self::empty())
    }
}

/// Get specified config values from config file.
///
/// # Errors
///
/// Forwards errors from [`PartialConfig::from_config_file`].
pub fn get_values_from_file<T: AsRef<str>>(value_names: &[T]) -> Result<Vec<(String, String)>> {
    let mut config_values: Vec<(String, String)> = Vec::new();
    let config = PartialConfig::from_config_file()?;
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
