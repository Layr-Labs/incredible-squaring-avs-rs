use std::collections::HashMap;
use std::hash::Hash;

use ark_bn254::G1Affine;
use ark_serialize::CanonicalSerialize;
use eigen_crypto_bls::Signature;
use eigen_types::avs::TaskResponseDigest;
use eigen_types::operator::OperatorId;
use incredible_bindings::IncredibleSquaringTaskManager;
use incredible_bindings::IncredibleSquaringTaskManager::TaskResponse;
use incredible_bindings::IncredibleSquaringTaskManager::{
    respondToTaskCall, G1Point, NewTaskCreated, Task, TaskResponded, TaskResponseMetadata,
};
use incredible_chainio::AvsWriter;
use incredible_config::IncredibleConfig;
use serde::Serialize;
use serde::Serializer;

// /// Wrapper on G1AFfine , as G1AFfine does not implement Serialize
// #[derive(Debug)]
// struct G1AffineWrapper(pub G1Affine);

// impl Serialize for G1AffineWrapper {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut serialized_bytes = vec![];
//         // using ark
//         self.0
//             .serialize_uncompressed(&mut serialized_bytes)
//             .map_err(serde::ser::Error::custom)?;
//         let hex_string = hex::encode(serialized_bytes);
//         // Serialize the hex string
//         serializer.serialize_str(&hex_string)
//     }
// }

/// Signed Task Response
#[derive(Debug)]
pub struct SignedTaskResponse {
    task_response: TaskResponse,
    signature: Signature,
    operator_id: OperatorId,
}

impl SignedTaskResponse {
    pub fn new(
        task_response: TaskResponse,
        bls_signature: Signature,
        operator_id: OperatorId,
    ) -> Self {
        Self {
            task_response,
            signature: bls_signature,
            operator_id,
        }
    }

    pub fn signature(&self) -> Signature {
        self.signature.clone()
    }
}

pub struct Aggregator {
    port_address: String,

    avs_writer: AvsWriter,

    // bls_aggregation_service:,
    tasks: HashMap<u32, IncredibleSquaringTaskManager::Task>,

    tasks_responses:
        HashMap<u32, HashMap<TaskResponseDigest, IncredibleSquaringTaskManager::TaskResponse>>,
}

impl Aggregator {
    // pub fn new(config:IncredibleConfig) -> Self{

    // }
}
