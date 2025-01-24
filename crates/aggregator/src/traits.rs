use alloy::primitives::B256;
use alloy::sol_types::SolEvent;
use eigen_services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigen_types::{avs::TaskIndex, operator::QuorumThresholdPercentages};
use serde::{Deserialize, Serialize};
use std::{future::Future, time::Duration};

// TODO: maybe `TaskMetadata` would be a better name
#[derive(Debug)]
/// Metadata related to a task. Used for signature aggregation.
pub struct TaskInfo {
    /// Index of the task
    pub task_index: TaskIndex,
    /// Block the task was created
    pub task_created_block: u32,
    /// Quorum numbers which should respond to the task
    pub quorum_nums: Vec<u8>,
    /// Thresholds for each quorum
    pub quorum_threshold_percentages: QuorumThresholdPercentages,
    /// Time before expiry of the task response aggregation
    pub time_to_expiry: Duration,
}

/// Abstracts task-specific behaviour
pub trait TaskProcessor {
    /// The event type expected by the task processor
    type NewTaskEvent: SolEvent + Send + Sync + 'static;

    /// The response type expected by the task processor
    type TaskResponse: TaskResponse + Send + Sync + 'static;

    /// Processes a task, returning metadata related to signature aggregation
    fn process_new_task(&self, event: Self::NewTaskEvent) -> impl Future<Output = TaskInfo> + Send;

    /// Processes a task response, returning the response's digest
    fn process_task_response(&self, event: Self::TaskResponse)
        -> impl Future<Output = B256> + Send;

    /// Process the result of a BLS aggregation
    fn process_aggregated_response(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> impl Future<Output = ()> + Send;
}

/// Task response trait
pub trait TaskResponse: for<'de> Deserialize<'de> + Serialize {
    /// Returns the index of the task
    fn task_index(&self) -> TaskIndex;
}
