use crate::{error::ChallengerError, TaskResponseData};
use alloy::consensus::{
    Signed, Transaction, TxEip4844, TxEip4844Variant, TxEnvelope, TypedTransaction,
};
use alloy::rpc::types::{Log, TransactionReceipt};
use alloy::signers::Signature;
use alloy::sol_types::SolCall;
use alloy::{hex::FromHex, primitives::TxHash, rpc::types::AccessList};
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::Task;
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::{
    respondToTaskCall, TaskResponded,
};
use incredible_bindings::incrediblesquaringtaskmanager::BN254::G1Point;
use incredible_chainio::fake_avs_writer::FakeAvsWriter;
use std::collections::HashMap;

/// Fake Challenger
#[derive(Debug)]
pub(crate) struct FakeChallenger {
    pub fake_avs_writer: FakeAvsWriter,
    #[allow(unused)]
    pub ws_url: String,
    #[allow(unused)]
    pub rpc_url: String,

    pub tasks: HashMap<u32, Task>,

    pub task_responses: HashMap<u32, TaskResponseData>,
}

#[allow(unused)]
impl FakeChallenger {
    /// Raise challenge
    pub(crate) async fn raise_challenge(&self, task_index: u32) -> Result<TxHash, ChallengerError> {
        let raise_challenge_result = self
            .fake_avs_writer
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
            Err(e) => Err(ChallengerError::ChainIo(e)),
        }
    }

    fn fake_transaction(&self) -> Result<Option<alloy::primitives::Bytes>, ChallengerError> {
        Ok(Some(alloy::primitives::Bytes::from_hex("0x5baec9a000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000006b0000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000018000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000001c0225554388e71f9eb5b46bd3813aabe8f3e2de4b965ff3727ada663f39b01f6e70f1c11ddf2169ba7b050f0cdc44223363a8912f0f3a9362d1d7ce0aa78fe864a2bb844ee415b8941017bd0f88d1f4e98a33ffaf917fdce5430ba36dbc5dfc1fe10d6ada6d8aa5fa3a4309df678bf6f370301f78e514b842ae50fea2afc54585c07aad5855f9e9f70a9076612ec3898ecb82d70cb0169b2ee59bbd44526914c7d117eaf682b217e97c383748202bc08091cf63f22da67328d10c185a39ab916d80000000000000000000000000000000000000000000000000000000000000220000000000000000000000000000000000000000000000000000000000000026000000000000000000000000000000000000000000000000000000000000002a0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001009d50828897fe208275d989abddcad762bf1bb1a089d5ad40ca5dc78e20faac256c79f6817fd79f3a4898e41b5212ccae66d5e9441c9c76f239a2966f24ba5e0000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000").unwrap()))
    }

    /// Process task response log
    pub(crate) async fn process_task_response_log(
        &mut self,
        task_response_log: Log,
    ) -> Result<u32, ChallengerError> {
        let non_signing_operator_pub_keys_result = self
            .get_non_signing_operator_pub_keys(task_response_log.clone())
            .await;

        match non_signing_operator_pub_keys_result {
            Ok(non_signing_operator_pub_key) => {
                let decoded_event = task_response_log.log_decode::<TaskResponded>().ok();
                if let Some(decoded) = decoded_event {
                    let data = decoded.data();

                    let task_response_data = TaskResponseData {
                        task_response: data.taskResponse.clone(),
                        task_response_metadata: data.taskResponseMetadata.clone(),
                        non_signing_operator_pub_keys: non_signing_operator_pub_key,
                    };

                    self.task_responses
                        .insert(data.taskResponse.referenceTaskIndex, task_response_data);

                    Ok(data.taskResponse.referenceTaskIndex)
                } else {
                    Err(ChallengerError::DecodeEvent)
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Get non signing operator pub keys
    pub(crate) async fn get_non_signing_operator_pub_keys(
        &self,
        log: Log,
    ) -> Result<Vec<G1Point>, ChallengerError> {
        let decoded_event = log.log_decode::<TaskResponded>().ok();
        if let Some(task_responded) = decoded_event {
            let tx_hash_result = task_responded.transaction_hash;
            if let Some(_tx_hash) = tx_hash_result {
                let transaction_data_result = self.fake_transaction();
                match transaction_data_result {
                    Ok(transaction_data_option) => {
                        if let Some(transaction_data) = transaction_data_option {
                            let calldata = transaction_data;
                            let decoded = respondToTaskCall::abi_decode(&calldata, false);

                            match decoded {
                                Ok(decoded) => {
                                    let non_signer_stakes_and_signature =
                                        decoded.nonSignerStakesAndSignature;

                                    let mut non_signing_operator_pub_keys: Vec<G1Point> = vec![];

                                    for (i, pub_key) in non_signer_stakes_and_signature
                                        .nonSignerPubkeys
                                        .iter()
                                        .enumerate()
                                    {
                                        non_signing_operator_pub_keys[i] = G1Point {
                                            X: pub_key.X,
                                            Y: pub_key.Y,
                                        };
                                    }
                                    Ok(non_signing_operator_pub_keys)
                                }

                                Err(e) => Err(ChallengerError::AlloySolType(e)),
                            }
                        } else {
                            Err(ChallengerError::TaskResponseNotFound)
                        }
                    }
                    Err(e) => Err(e),
                }
            } else {
                Err(ChallengerError::TransactionHashNotFound)
            }
        } else {
            Err(ChallengerError::EmptyDecodedData)
        }
    }
}
