use eigen_services_blsaggregation::bls_agg::BlsAggregationServiceError;
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
}
