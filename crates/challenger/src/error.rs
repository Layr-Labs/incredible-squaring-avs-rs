use alloy::contract::Error as AlloyError;
use alloy::sol_types::Error as AlloySolTypeError;
use incredible_chainio::error::ChainIoError;
use incredible_config::error::ConfigError;
use thiserror::Error;

/// Error returned by chainio
#[derive(Debug, Error)]
pub enum ChallengerError {
    #[error("Chain Io crate error : {0}")]
    ChainIo(#[from] ChainIoError),

    #[error("Alloy contract error: {0}")]
    AlloyContractError(#[from] AlloyError),

    #[error("Task Response is not wrong")]
    TaskResponseisCorrect,

    #[error("Task Response not found")]
    TaskResponseNotFound,

    #[error("Task Response not found")]
    TaskNotFound,

    #[error("Alloy sol types error :{0}")]
    AlloySolType(#[from] AlloySolTypeError),

    #[error("Tx hash not found")]
    TransactionHashNotFound,

    #[error("Decoded data empty")]
    EmptyDecodedData,

    #[error("Failed to decode event ")]
    DecodeEvent,

    /// Failed to parse ECDSA keystore signer
    #[error("Failed to parse ecdsa keystore signer")]
    ECDSAKeystoreSigner,

    #[error("Config error {0}")]
    ConfigParseError(#[from] ConfigError),
}
