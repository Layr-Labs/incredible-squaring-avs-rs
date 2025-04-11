//! Challenger crate

use alloy::consensus::Transaction;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::rpc::types::Log;
use alloy::sol_types::SolCall;
use async_trait::async_trait;
use eigensdk::challenger::challenger::ChallengerTaskProcessor;
use eigensdk::challenger::error::ChallengerError;
use eigensdk::common::get_provider;
use incredible_aggregator::IncredibleTaskResponse;
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::{
    Task, TaskResponseMetadata,
};
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::{
    respondToTaskCall, NewTaskCreated, TaskResponded,
};
use incredible_bindings::incrediblesquaringtaskmanager::BN254::G1Point;
use incredible_chainio::AvsWriter;
use std::collections::HashMap;

/// Task Response Data
#[derive(Debug)]
pub struct TaskResponseData {
    task_response: IncredibleTaskResponse,
    task_response_metadata: TaskResponseMetadata,
    non_signing_operator_pub_keys: Vec<G1Point>,
}

/// Challenger Task Processor Implementation
#[derive(Debug)]
pub struct ChallengerTaskProcessorImpl {
    tasks: HashMap<u32, Task>,
    task_responses: HashMap<u32, TaskResponseData>,
    rpc_url: String,
    avs_writer: AvsWriter,
}

/// 1. Implement ChallengerTaskProcessor trait
#[async_trait]
impl ChallengerTaskProcessor for ChallengerTaskProcessorImpl {
    /// Event type for the new task
    type NewTaskEvent = NewTaskCreated;
    /// Event type for the task response
    type TaskResponseEvent = TaskResponded;

    async fn handle_task_creation(&mut self, decoded: Log<Self::NewTaskEvent>) {
        dbg!("handle_task_creation");
        let data = decoded.data();
        self.tasks.insert(data.taskIndex, data.task.clone());
    }

    async fn handle_task_response(&mut self, decoded: Log<Self::TaskResponseEvent>) {
        dbg!("handle_task_response");
        let non_signing_operator_pub_keys = self
            .get_non_signing_operator_pub_keys(decoded.clone())
            .await
            .unwrap();

        let task_index = decoded.data().taskResponse.referenceTaskIndex;
        let task_response = decoded.data().taskResponse.clone();
        let task_response_metadata = decoded.data().taskResponseMetadata.clone();

        self.task_responses.insert(
            task_index,
            TaskResponseData {
                task_response: IncredibleTaskResponse(task_response),
                task_response_metadata,
                non_signing_operator_pub_keys,
            },
        );

        if self.tasks.contains_key(&task_index) && !self.valid_task_response(task_index) {
            dbg!("ENTRO A RAISE CHALLENGE");
            self.raise_challenge(task_index).await;
        }
    }
}

impl ChallengerTaskProcessorImpl {
    pub async fn new(rpc_url: String, service_manager_addr: Address, signer: String) -> Self {
        let avs_writer = AvsWriter::new(service_manager_addr, rpc_url.clone(), signer)
            .await
            .unwrap();

        Self {
            tasks: HashMap::new(),
            task_responses: HashMap::new(),
            rpc_url,
            avs_writer,
        }
    }

    fn valid_task_response(&self, task_index: u32) -> bool {
        let task = self.tasks.get(&task_index).unwrap();
        let num_to_square = task.numberToBeSquared;

        let task_response = self.task_responses.get(&task_index).unwrap();
        let answer = task_response.task_response.0.numberSquared;

        answer == num_to_square * num_to_square
    }

    async fn raise_challenge(&self, task_index: u32) {
        let raise_challenge_result = self
            .avs_writer
            .raise_challenge(
                self.tasks[&task_index].clone(),
                self.task_responses[&task_index].task_response.0.clone(),
                self.task_responses[&task_index]
                    .task_response_metadata
                    .clone(),
                self.task_responses[&task_index]
                    .non_signing_operator_pub_keys
                    .clone(),
            )
            .await
            .unwrap();
    }

    pub async fn get_non_signing_operator_pub_keys(
        &self,
        log: alloy::rpc::types::Log<TaskResponded>,
    ) -> Result<Vec<G1Point>, ChallengerError> {
        let Some(tx_hash) = log.transaction_hash else {
            return Err(ChallengerError::TransactionHashNotFound);
        };

        let provider = get_provider(&self.rpc_url);
        let transaction = provider.get_transaction_by_hash(tx_hash).await?.unwrap();
        let calldata = transaction.inner.input();

        let decoded = respondToTaskCall::abi_decode(calldata, false);
        if let Ok(decoded) = decoded {
            let non_signer_stakes_and_signature = decoded.nonSignerStakesAndSignature;
            let non_signing_operator_pub_keys = non_signer_stakes_and_signature
                .nonSignerPubkeys
                .iter()
                .map(|pub_key| G1Point {
                    X: pub_key.X,
                    Y: pub_key.Y,
                })
                .collect();
            Ok(non_signing_operator_pub_keys)
        } else {
            Err(ChallengerError::AlloySolType(decoded.err().unwrap()))
        }
    }
}
