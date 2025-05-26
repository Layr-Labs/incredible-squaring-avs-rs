use serde::de::DeserializeOwned;
use std::{fs, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Toml error: {0}")]
    Toml(#[from] toml::de::Error),
}

/// Loads a config from a file
///
/// # Arguments
///
/// * `path` - The path to the config file
///
/// # Returns
///
/// * `Result<T, ConfigError>` - The config struct
pub fn load_config<P, T>(path: P) -> Result<T, ConfigError>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let s = fs::read_to_string(&path)?;
    toml::from_str(&s).map_err(ConfigError::Toml)
}
