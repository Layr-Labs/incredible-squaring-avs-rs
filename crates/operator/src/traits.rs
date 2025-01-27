use alloy::{
    primitives::{keccak256, Address},
    providers::WsConnect,
    rpc::types::Filter,
    sol_types::{SolEvent, SolValue},
};

pub trait Operator {
    /// Sign the task response
    fn sign_task_response(
        &self,
        task_response: TaskResponse,
    ) -> Result<SignedTaskResponse, OperatorError> {
        let encoded_response = TaskResponse::abi_encode(&task_response);
        let hash_msg = keccak256(encoded_response);

        let signed_msg = self.key_pair.sign_message(&hash_msg);
        let signed_task_response =
            SignedTaskResponse::new(task_response, signed_msg, self.operator_id);
        Ok(signed_task_response)
    }
}
