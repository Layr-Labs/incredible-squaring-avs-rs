use alloy::primitives::{Address,U256};
use eigen_client_avsregistry::{error::AvsRegistryError, writer::AvsRegistryChainWriter};
use error::ChainIoError;
pub mod error;
use eigen_types::operator::{QuorumThresholdPercentage,QuorumNum};
use incredible_bindings::{IncredibleSquaringServiceManager};
use eigen_utils::{binding::{RegistryCoordinator},get_provider};

#[derive(Debug)]
pub struct AvsWriter {

    avs_registry_writer : AvsRegistryChainWriter,
    service_task_manager_addr: Address,
}

impl AvsWriter {
    pub async fn new(
        registry_coordinator_addr: Address,
        operator_state_retriever_addr: Address,
        rpc_url: String,
        signer: String
    ) -> Result<Self, error::ChainIoError> {
         let writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
            rpc_url,
            signer,
            registry_coordinator_addr,
            operator_state_retriever_addr
        )
        .await;
        match writer {
            Ok(avs_writer) =>{
                let provider = get_provider(&rpc_url);
                let contract_registry_coordinator = RegistryCoordinator::new(registry_coordinator_addr,&provider);
                let service_manager_addr_return = contract_registry_coordinator.serviceManager();

                let {_0: service_manager_addr} = service_manager_addr_return;
                // let new_task_service_manager_addr = IncredibleSquaringServiceManager::new(service_manager_addr,&provider);
                Ok(AvsWriter{avs_registry_writer: avs_writer,service_task_manager_addr:})
            }
            Err(e) => Err(ChainIoError::AvsWriterBuildFail { reason: e.to_string() })
            
        }

    }

    pub fn send_task_number_to_square(&self,num_to_square:U256,quorum_threshold_percentages:QuorumThresholdPercentage,quorum_nums:Vec<QuorumNum>) {



    }






}
