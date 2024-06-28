use crate::error::OperatorError::{CargoManifestDir, ECDSAKeystoreSigner};
use alloy::signers::local::LocalSigner;
use core::task;
use incredible_bindings::IncredibleSquaringTaskManager::{self, NewTaskCreated, TaskResponse};
use incredible_config::IncredibleConfig;
use std::{env, path::PathBuf};
/// Main Operator
#[derive(Debug, Default)]
pub struct OperatorBuilder {
    rpc_url: String,
}

impl OperatorBuilder {
    pub fn build(&self, config: IncredibleConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Read ECDSA private key from path
        let cargo_dir_result = env::var("CARGO_MANIFEST_DIR");

        match cargo_dir_result {
            Ok(cargo_dir) => {
                println!("key store path file : {:?}", config.ecdsa_keystore_path());
                let signer_result = LocalSigner::decrypt_keystore(
                    config.ecdsa_keystore_path(),
                    config.ecdsa_keystore_password(),
                );

                match signer_result {
                    Ok(signer) => {
                        println!("signer : {signer:?}");

                        // TODO Bls keystore
                        // Register here or using cli?

                        todo!()
                    }
                    Err(e) => {
                        println!("Error is {e:?}");
                        Err(Box::new(ECDSAKeystoreSigner))
                    }
                }
            }
            Err(e) => Err(Box::new(CargoManifestDir)),
        }
    }

    // 2335, 2333, 2334 eips for bls keystore

    pub fn process_new_task(&self, new_task_created: NewTaskCreated) -> TaskResponse {
        let num_squared =
            new_task_created.task.numberToBeSquared * new_task_created.task.numberToBeSquared;

        let task_response = TaskResponse {
            referenceTaskIndex: new_task_created.taskIndex,
            numberSquared: num_squared,
        };
        task_response
    }

    pub fn start_operator(&self) {
        todo!()
    }

    pub fn sign_task_response(&self) {
        todo!()
    }
}
