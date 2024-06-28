use thiserror::Error;

/// Error returned by AvsRegistry
#[derive(Debug, Error)]
pub enum OperatorError {
    /// Failed to parse ECDSA keystore signer
    #[error("Failed to parse ecdsa keystore signer")]
    ECDSAKeystoreSigner,

    /// Failed to derive Cargo Manfest Dir
    #[error("Could not derive cargo manifest path")]
    CargoManifestDir,
}
