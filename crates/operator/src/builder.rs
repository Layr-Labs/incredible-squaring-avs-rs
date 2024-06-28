
use std::{env, path::PathBuf};
use alloy::signers::local::LocalSigner;
use incredible_config::IncredibleConfig;

/// Main Operator 
#[derive(Debug,Default)]
pub struct OperatorBuilder {

    rpc_url : String,




}


impl OperatorBuilder{

    pub fn build(&self,config: IncredibleConfig) -> Result<Self, Box<dyn std::error::Error>>{

    //     // Read ECDSA private key from path 
    //    let password = "";
    //    let keystore_path = "";
    //    let cargo_dir_result = env::var("CARGO_MANIFEST_DIR");

    //    match cargo_dir_result{

    //     Ok(cargo_dir) =>{

    //         let keystore_file_path =
    //         PathBuf::from(cargo_dir).join(keystore_path);
    
    //         let signer_result = LocalSigner::decrypt_keystore(keystore_file_path, password);

    //         match signer_result{

    //             Ok(signer) =>{
                    
    //             },
    //             Err(e) =>{
    //                 Ok(Box::new(e.to_string()))
    //             }

    //         }
    //     },
    //     Err(e) =>{
    //         Ok(Box::new(e.to_string()))
    //     }


    todo!()

       }



       
// 2335, 2333, 2334


    

    pub fn process_new_task(&self) -> Self{



todo!()
    }

    pub fn start_operator(&self) {

        todo!()

    }



    pub fn sign_task_response(&self) {

        todo!()
    }

}
