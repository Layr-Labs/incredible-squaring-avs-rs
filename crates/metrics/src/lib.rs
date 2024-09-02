//! Metrics for the incredible squaring task manager
use metrics::{counter, describe_counter};

/// Create a new instance of the metrics
pub fn new() {
    describe_counter!(
        "num_tasks_received",
        "The number of tasks received by reading from the avs service manager contract"
    );
    describe_counter!(
        "num_signed_task_responses_accepted_by_aggregator",
        "The number of signed task responses accepted by the aggregator"
    );
}

/// Increment the number of tasks received
pub fn increment_num_tasks_received() {
    counter!("num_tasks_received").increment(1);
}

/// Increment the number of tasks received
pub fn inc_num_tasks_accepted_by_aggregator() {
    counter!("num_signed_task_responses_accepted_by_aggregator").increment(1);
}
