use alloy::contract::Error as AlloyError;
use incredible_chainio::error::ChainIoError;
use thiserror::Error;

/// Error returned by chainio
#[derive(Debug, Error)]
pub enum ChallengerError {
    #[error("Chain Io crate error : {0}")]
    ChainIo(#[from] ChainIoError),

    #[error("Alloy contract error: {0}")]
    AlloyContractError(#[from] AlloyError),
}
