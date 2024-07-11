use alloy::contract::Error as AlloyError;
use thiserror::Error;

/// Error returned by chainio
#[derive(Debug, Error)]
pub enum ChainIoError {
    #[error("Avs writer build fail : {reason}")]
    AvsWriterBuildFail { reason: String },

    #[error("Alloy contract error: {0}")]
    AlloyContractError(#[from] AlloyError),

    #[error("No logs generated in Create new task function")]
    CreateNewTaskNoEventFound,
}
