//! Challenger crate
use std::collections::HashMap;

pub mod error;
use alloy::rpc::types::TransactionReceipt;
use alloy::rpc::types::{BlockNumberOrTag, Filter};
use alloy::sol_types::SolEvent;
use alloy_provider::{Provider, ProviderBuilder, WsConnect};
use error::ChallengerError;
use eyre::Result;
use futures_util::stream::StreamExt;
use incredible_bindings::IncredibleSquaringTaskManager::{
    G1Point, NewTaskCreated, Task, TaskResponded, TaskResponse, TaskResponseMetadata,
};
use incredible_chainio::AvsWriter;
use incredible_config::IncredibleConfig;
use tracing::info;

/// Task Response Data
#[derive(Debug)]
pub struct TaskResponseData {
    task_response: TaskResponse,
    task_response_metadata: TaskResponseMetadata,
    non_signing_operator_pub_keys: Vec<G1Point>,
}

#[derive(Debug)]
pub struct Challenger {
    avs_writer: AvsWriter,

    ws_url: String,

    rpc_url: String,

    tasks: HashMap<u32, Task>,

    task_responses: HashMap<u32, TaskResponseData>,
}

impl Challenger {
    pub fn new(config: IncredibleConfig) -> Self {
        // let avs_writer
        todo!()
    }

    /// Start the challenger
    pub async fn start_challenger(&mut self) -> Result<(), ChallengerError> {
        info!("Starting Challenger.");
        info!("Subscribed to new tasks");

        let ws = WsConnect::new(self.ws_url.clone());
        let provider = ProviderBuilder::new().on_ws(ws).await?;
        let new_task_created_filter = Filter::new()
            .event("NewTaskCreated(uint32,(uint256,uint32,bytes,uint32))")
            .from_block(BlockNumberOrTag::Latest);
        let new_task_created_sub = provider.subscribe_logs(&new_task_created_filter).await?;
        let mut new_task_created_stream = new_task_created_sub.into_stream();

        let new_task_created_log = NewTaskCreated::SIGNATURE_HASH;

        let task_responded_filter = Filter::new()
            .event("TaskResponded((uint32,uint256),(uint32,bytes32))")
            .from_block(BlockNumberOrTag::Latest);
        let task_responded_sub = provider.subscribe_logs(&task_responded_filter).await?;

        let mut task_responded_stream = task_responded_sub.into_stream();

        let task_responded_log = TaskResponded::SIGNATURE_HASH;

        for _ in 0..2 {
            let log = tokio::select! {

                Some(log) = task_responded_stream.next() =>{
                   log
                },
                Some(log) = new_task_created_stream.next() =>{
                    log
                }

            };

            let topic = log.topic0();

            if let Some(tp) = topic {
                if *tp == new_task_created_log {
                    let new_task_created_option = log.log_decode::<NewTaskCreated>().ok();

                    if let Some(data) = new_task_created_option {
                        let m = data.data();
                        let new_task_cr = NewTaskCreated {
                            taskIndex: m.taskIndex,
                            task: m.task.clone(),
                        };

                        let t_index = self.process_new_task_created_log(new_task_cr);

                        if let Some(_) = self.task_responses.get(&t_index) {
                            let call_c_result = self.call_challenge(t_index);
                            match call_c_result {
                                Ok(call_c) => continue,
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                        }
                    }
                } else if *tp == task_responded_log {
                }
            }
        }

        Ok(())
    }

    pub fn process_new_task_created_log(&mut self, new_task_created_log: NewTaskCreated) -> u32 {
        self.tasks
            .insert(new_task_created_log.taskIndex, new_task_created_log.task);

        new_task_created_log.taskIndex
    }

    pub fn call_challenge(&self, task_index: u32) -> Result<(), ChallengerError> {
        if let Some(number_to_be_squared) = self.tasks.get(&task_index) {
            let num_to_square = number_to_be_squared.numberToBeSquared;

            if let Some(answer_in_response) = self.task_responses.get(&task_index) {
                let answer = answer_in_response.task_response.numberSquared;

                if answer != (num_to_square * num_to_square) {
                    self.raise_challenge(task_index);
                    return Ok(());
                }

                return Err(ChallengerError::TaskResponseisCorrect);
            } else {
                return Err(ChallengerError::TaskResponseNotFound);
            }
        } else {
            return Err(ChallengerError::TaskNotFound);
        }
    }

    pub async fn raise_challenge(
        &self,
        task_index: u32,
    ) -> Result<TransactionReceipt, ChallengerError> {
        let raise_challenge_result = self
            .avs_writer
            .raise_challenge(
                self.tasks[&task_index].clone(),
                self.task_responses[&task_index].task_response.clone(),
                self.task_responses[&task_index]
                    .task_response_metadata
                    .clone(),
                self.task_responses[&task_index]
                    .non_signing_operator_pub_keys
                    .clone(),
            )
            .await;
        match raise_challenge_result {
            Ok(raise_challenge) => Ok(raise_challenge),
            Err(e) => Err(error::ChallengerError::ChainIo(e)),
        }
    }
}
