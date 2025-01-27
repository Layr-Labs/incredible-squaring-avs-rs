use alloy::contract::Error as AlloyError;
use alloy::sol_types::Error as AlloySolTypeError;
use eigen_config::error::ConfigError;
use incredible_chainio::error::ChainIoError;
use thiserror::Error;

/// Error returned by chainio
#[derive(Debug, Error)]
pub enum ChallengerError {
    /// Chain Io crate error
    #[error("Chain Io crate error : {0}")]
    ChainIo(#[from] ChainIoError),

    /// Alloy contract error
    #[error("Alloy contract error: {0}")]
    AlloyContractError(#[from] AlloyError),

    /// Task Response is correct
    #[error("Task Response is not wrong")]
    TaskResponseisCorrect,

    /// Task Response not found
    #[error("Task Response not found")]
    TaskResponseNotFound,

    /// Task not found
    #[error("Task not found")]
    TaskNotFound,

    /// Alloy sol types error
    #[error("Alloy sol types error :{0}")]
    AlloySolType(#[from] AlloySolTypeError),

    /// Transaction hash not found
    #[error("Tx hash not found")]
    TransactionHashNotFound,

    /// Decoded data empty
    #[error("Decoded data empty")]
    EmptyDecodedData,

    /// Failed to decode event
    #[error("Failed to decode event ")]
    DecodeEvent,

    /// Failed to parse ECDSA keystore signer
    #[error("Failed to parse ecdsa keystore signer")]
    ECDSAKeystoreSigner,

    /// Config error
    #[error("Config error {0}")]
    ConfigParseError(#[from] ConfigError),

    /// Toml Error
    #[error("Toml error")]
    Toml(#[from] toml::de::Error),
}
