//! Aggregator crate
//!
//! Implementation of the trait TaskProcessor and TaskResponse for the Incredible Squaring AVS

/// Implementation of the trait TaskProcessor from the eigensdk crate
pub mod task_processor;
/// Implementation of the trait TaskResponse from the eigensdk crate
pub mod task_response;

pub use task_processor::IncredibleTaskProcessor;
pub use task_response::IncredibleTaskResponse;
