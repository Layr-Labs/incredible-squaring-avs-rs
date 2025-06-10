//! Utility functions for the incredible squaring example.

use eigensdk::logging::{log_level::LogLevel, logger::Logger, tracing_logger::TracingLogger};
use serde::de::DeserializeOwned;
use std::{fs, path::Path, sync::Arc};
use thiserror::Error;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Error)]
pub enum UtilsError {
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
/// * `Result<T, UtilsError>` - The config struct
pub fn load_config<P, T>(path: P) -> Result<T, UtilsError>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let s = fs::read_to_string(&path)?;
    toml::from_str(&s).map_err(UtilsError::Toml)
}

/// Creates a logger with the given level
/// This will disable tarpc logging and set the default level for all other loggers
///
/// # Arguments
///
/// * `level` - The level of the logger
///
/// # Returns
///
/// * `Arc<dyn Logger>` - The logger
pub fn create_logger(level: LogLevel) -> Arc<dyn Logger> {
    let tracing_level = match level {
        LogLevel::Fatal => tracing::Level::ERROR,
        LogLevel::Error => tracing::Level::ERROR,
        LogLevel::Warn => tracing::Level::WARN,
        LogLevel::Info => tracing::Level::INFO,
        LogLevel::Debug => tracing::Level::DEBUG,
        LogLevel::Trace => tracing::Level::TRACE,
    };

    // Disable tarpc logging
    let mut filter = EnvFilter::new("tarpc=off");
    // Set the default level for all other loggers
    filter = filter.add_directive(tracing_level.into());

    let subscriber = tracing_subscriber::fmt().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    Arc::new(TracingLogger {
        level,
        ..Default::default()
    })
}
