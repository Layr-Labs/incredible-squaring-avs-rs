use alloy::contract::Error as ContractError;
use alloy::transports::{RpcError, TransportErrorKind};
use eigensdk::client_avsregistry::error::AvsRegistryError;
use eigensdk::crypto_bls::error::BlsError;
use eigensdk::services_blsaggregation::bls_aggregation_service_error::BlsAggregationServiceError;
use eigensdk::services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceError;
use incredible_chainio::error::ChainIoError;
use incredible_config::error::ConfigError;
use serde_json::Error;
use thiserror::Error;

/// Error returned by chainio
#[derive(Debug, Error)]
pub enum AggregatorError {
    /// Bls Aggregation Service Error
    #[error("Bls Aggregation Service Error : {0}")]
    BlsAggregationServiceError(#[from] BlsAggregationServiceError),
    /// Task Response not found
    #[error("Task Response not found")]
    TaskResponseNotFound,
    /// parse error
    #[error("Config parse error")]
    ParseError(#[from] ConfigError),
    /// Build avs registry chain reader
    #[error("Failed to build avs registry chain reader ")]
    BuildAvsRegistryChainReader(#[from] AvsRegistryError),

    /// build avswriter
    #[error("Failed to build avs wrtier in chain io ")]
    BuildAvsWriter(#[from] ChainIoError),

    /// Bls crate error
    #[error("Bls Crate Error SDK")]
    Bls(#[from] BlsError),

    /// alloy rpc error
    #[error("Alloy rpc error")]
    AlloyRpc(#[from] RpcError<TransportErrorKind>),

    /// serde json error
    #[error("Serde json error")]
    SerdeError(#[from] Error),

    /// IO error
    #[error("IO error")]
    IOError(#[from] std::io::Error),

    /// Operator Info service error
    #[error("Operator Info Service error")]
    OperatorInfoServiceError(#[from] OperatorInfoServiceError),

    /// Alloy Contract Error
    #[error("Alloy Contract Error")]
    ContractError(#[from] ContractError),
}
