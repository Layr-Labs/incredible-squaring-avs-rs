use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_logging::get_logger;
use incredible_config::IncredibleConfig;

use crate::error::ChainIoError;

/// Reader for the AVS Registry
#[derive(Debug)]
pub struct AvsReader {
    avs_registry: AvsRegistryChainReader,
}

impl AvsReader {
    /// Builds [`AvsReader`]
    pub async fn build(config: IncredibleConfig) -> Result<Self, ChainIoError> {
        let registry_coordinator_address = config.registry_coordinator_addr()?;
        let operator_state_retriever_address = config.operator_state_retriever_addr()?;
        let http_rpc_url = config.http_rpc_url();
        let avs_registry_chain_reader = AvsRegistryChainReader::new(
            get_logger(),
            registry_coordinator_address,
            operator_state_retriever_address,
            http_rpc_url,
        )
        .await?;
        Ok(Self {
            avs_registry: avs_registry_chain_reader,
        })
    }
}
