use super::error::OperatorError;
use alloy::{primitives::keccak256, sol_types::SolValue};
use eigen_crypto_bls::BlsKeyPair;
use eigen_types::operator::OperatorId;
// TODO: change this for the generic version.
use incredible_aggregator::SignedTaskResponse;
// TODO: change this for the generic version.
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::TaskResponse;

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
