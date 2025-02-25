use alloy::contract::Error as ContractError;
use alloy::{
    providers::PendingTransactionError as AlloyError,
    signers::local::LocalSignerError,
    transports::{RpcError, TransportErrorKind},
};
use eigensdk::client_avsregistry::error::AvsRegistryError;
use incredible_config::error::ConfigError;
use thiserror::Error;
/// Error returned by chainio
#[derive(Debug, Error)]
pub enum ChainIoError {
    /// Avs writer build fail
    #[error("Avs writer build fail : {reason}")]
    AvsWriterBuildFail {
        /// The reason for the build failure
        reason: String,
    },
    /// Alloy contract error
    #[error("Alloy provider error: {0}")]
    AlloyProviderError(#[from] AlloyError),
    /// No logs generated in Create new task function
    #[error("No logs generated in Create new task function")]
    CreateNewTaskNoEventFound,
    /// Config error
    #[error("Config error {0}")]
    ConfigParseError(#[from] ConfigError),
    /// Avs registry error in eigensdk-rs
    #[error("AvsRegistry error in eigensdk-rs")]
    SdkAvsRegistryChainError(#[from] AvsRegistryError),
    /// Alloy Rpc Error
    #[error("Alloy Rpc Error")]
    RpcError(#[from] RpcError<TransportErrorKind>),
    /// Alloy Signer Error
    #[error("Alloy Signer Error")]
    SignerError(#[from] LocalSignerError),
    /// Alloy Contract Error
    #[error("Alloy Contract Error")]
    ContractError(#[from] ContractError),
}
