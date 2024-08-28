use eigen_services_blsaggregation::bls_agg::BlsAggregationServiceError;
use thiserror::Error;

/// Error returned by chainio
#[derive(Debug, Error)]
pub enum AggregatorError {
    #[error("Bls Aggregation Service Error : {0}")]
    BlsAggregationServiceError(#[from] BlsAggregationServiceError),

    #[error("Task Response not found")]
    TaskResponseNotFound,
}
