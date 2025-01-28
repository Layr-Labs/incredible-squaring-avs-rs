use super::error::OperatorError;
use alloy::primitives::Address;
use alloy::{primitives::keccak256, sol_types::SolValue};
use eigen_crypto_bls::BlsKeyPair;
use eigen_types::operator::OperatorId;
// TODO: change this for the generic version.
use incredible_aggregator::SignedTaskResponse;
// TODO: change this for the generic version.
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eyre::Result;
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::TaskResponse;
// TODO: change this for the generic version.
use crate::client::ClientAggregator;
use alloy::{providers::WsConnect, rpc::types::Filter, sol_types::SolEvent};
use alloy_provider::{Provider, ProviderBuilder};
use futures_util::StreamExt;
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::{
    self, NewTaskCreated,
};
use std::sync::Arc;
use tracing::info;

/// Start the operator
pub async fn start_operator(
    avs_registry_reader: &AvsRegistryChainReader,
    key_pair: &BlsKeyPair,
    operator_id: &OperatorId,
    operator_address: Address,
    operator_name: &str,
    client_aggregator: &ClientAggregator,
    ws_rpc_url: &str,
) -> Result<()> {
    let is_registered = avs_registry_reader
        .is_operator_registered(operator_address)
        .await?;
    info!("is {} registered {}", operator_name, is_registered);
    let arc_client = Arc::new(client_aggregator);
    if is_registered {
        info!("Starting operator");

        let ws = WsConnect::new(ws_rpc_url);
        let provider = ProviderBuilder::new().on_ws(ws).await?;

        let filter = Filter::new().event_signature(NewTaskCreated::SIGNATURE_HASH);
        let sub = provider.subscribe_logs(&filter).await?;
        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            let task_option = log
                .log_decode::<IncredibleSquaringTaskManager::NewTaskCreated>()
                .ok();
            if let Some(task) = task_option {
                let data = task.data();
                let new_task_created = NewTaskCreated {
                    task: data.task.clone(),
                    taskIndex: data.taskIndex,
                };
                info!(
                    "{} picked up a new task, index: {} ",
                    operator_name, data.taskIndex
                );
                incredible_metrics::increment_num_tasks_received();
                let task_response = process_new_task(new_task_created);
                let signed_task_response =
                    sign_task_response(key_pair, operator_id, task_response)?;
                let _ = arc_client
                    .send_signed_task_response(signed_task_response)
                    .await;
            }
        }
    }
    Ok(())
}

/// Processes new task
// TODO! generalize this function
pub fn process_new_task(new_task_created: NewTaskCreated) -> TaskResponse {
    #[allow(unused_mut)]
    #[allow(unused_assignments)]
    let mut number_to_be_squared = new_task_created.task.numberToBeSquared;

    #[cfg(feature = "integration_tests")]
    {
        number_to_be_squared = alloy::primitives::U256::from(9);
        info!("Challenger test: setting number to be squared to 9");
    }

    let num_squared = number_to_be_squared * number_to_be_squared;

    TaskResponse {
        referenceTaskIndex: new_task_created.taskIndex,
        numberSquared: num_squared,
    }
}

/// Sign the task response
pub fn sign_task_response(
    key_pair: &BlsKeyPair,
    operator_id: &OperatorId,
    task_response: TaskResponse,
) -> Result<SignedTaskResponse, OperatorError> {
    let encoded_response = TaskResponse::abi_encode(&task_response);
    let hash_msg = keccak256(encoded_response);

    let signed_msg = key_pair.sign_message(&hash_msg);
    let signed_task_response = SignedTaskResponse::new(task_response, signed_msg, *operator_id);
    Ok(signed_task_response)
}
