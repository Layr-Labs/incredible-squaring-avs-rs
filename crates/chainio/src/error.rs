use thiserror::Error;

/// Error returned by chainio
#[derive(Debug, Error)]
pub enum ChainIoError {
    #[error("Avs writer build fail : {reason}")]
    AvsWriterBuildFail { reason: String },
}
