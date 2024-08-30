use alloy::contract::Error as AlloyError;
use eigen_client_avsregistry::error::AvsRegistryError;
use incredible_config::error::ConfigError;
use thiserror::Error;
/// Error returned by chainio
#[derive(Debug, Error)]
pub enum ChainIoError {
    /// Avs writer build fail
    #[error("Avs writer build fail : {reason}")]
    AvsWriterBuildFail { reason: String },
    /// Alloy contract error
    #[error("Alloy contract error: {0}")]
    AlloyContractError(#[from] AlloyError),
    /// No logs generated in Create new task function
    #[error("No logs generated in Create new task function")]
    CreateNewTaskNoEventFound,
    /// Config error
    #[error("Config error {0}")]
    ConfigParseError(#[from] ConfigError),
    /// Avs registry error in eigensdk-rs
    #[error("AvsRegistry error in eigensdk-rs")]
    SdkAvsRegistryChainError(#[from] AvsRegistryError),
}
