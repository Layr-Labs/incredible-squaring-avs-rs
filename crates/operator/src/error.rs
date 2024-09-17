use alloy::signers::local::LocalSignerError;
use eigen_crypto_bls::error::BlsError;
use incredible_config::error::ConfigError;
use rust_bls_bn254::errors::KeystoreError;
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
    /// Failed to build avsregistry reader
    #[error("Failed to build avsregistry reader")]
    AvsRegistryChainReader,
    /// Could not sign the hash using keypair
    #[error("Could not sign the hash using keypair")]
    SignUsingBlsKeyPair,
    /// Failed to create Encoded bls keystore
    #[error("Failed to create Encoded bls keystore ")]
    EncodedKeystore,
    /// Failed to parse bls keystore path
    #[error("failed to parse bls keystore path ")]
    BlsKeystorePath,
    /// Failed to parse config
    #[error("Config error {0}")]
    ConfigParseError(#[from] ConfigError),
    /// Local signer error
    #[error("Local signer error ")]
    AlloySignerError(#[from] LocalSignerError),
    /// Bls Keystore error
    #[error("Bls Keystore error ")]
    BlsKeystoreError(#[from] KeystoreError),
    /// Bls crate(SDK) error
    #[error("Bls crate(SDK) error")]
    EigenBlsError(#[from] BlsError),
}
