use crate::error::AggregatorError;
use alloy::providers::Provider;
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::rpc::types::Filter;
use alloy::sol_types::SolEvent;
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_crypto_bls::{convert_to_g1_point, convert_to_g2_point};
use eigen_logging::get_test_logger;
use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
use eigen_services_blsaggregation::bls_agg::{
    BlsAggregationServiceError, BlsAggregationServiceResponse, BlsAggregatorService,
};
use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use eigen_types::avs::TaskResponseDigest;
use eigen_utils::get_ws_provider;
use futures_util::StreamExt;
use incredible_bindings::incrediblesquaringtaskmanager::IBLSSignatureChecker::NonSignerStakesAndSignature;
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::{
    Task, TaskResponse,
};
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
use incredible_bindings::incrediblesquaringtaskmanager::BN254::{G1Point, G2Point};
use incredible_config::IncredibleConfig;
use jsonrpc_core::serde_json;
use jsonrpc_core::{Error, IoHandler, Params, Value};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

/// Task Challenge Window Block : 100 blocks
pub const TASK_CHALLENGE_WINDOW_BLOCK: u32 = 100;
/// Block Time Seconds : 12 seconds
pub const BLOCK_TIME_SECONDS: u32 = 12;

/// Aggregator
#[derive(Debug)]
pub struct FakeAggregator {
    port_address: String,
    bls_aggregation_service: BlsAggregatorService<
        AvsRegistryServiceChainCaller<AvsRegistryChainReader, OperatorInfoServiceInMemory>,
    >,
    /// HashMap to store tasks
    pub tasks: HashMap<u32, Task>,
    /// HashMap to store task responses
    pub tasks_responses: HashMap<u32, HashMap<TaskResponseDigest, TaskResponse>>,
}

impl FakeAggregator {
    /// Creates a new FakeAggregator
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the FakeAggregator
    ///
    /// # Returns
    ///
    /// * `Self` - The FakeAggregator
    pub async fn new(config: IncredibleConfig) -> Self {
        let avs_registry_chain_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            config.registry_coordinator_addr().unwrap(),
            config.operator_state_retriever_addr().unwrap(),
            config.http_rpc_url(),
        )
        .await
        .unwrap();

        let avs_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            config.registry_coordinator_addr().unwrap(),
            config.operator_state_retriever_addr().unwrap(),
            config.http_rpc_url(),
        )
        .await
        .unwrap();

        let operators_info_service = OperatorInfoServiceInMemory::new(
            get_test_logger(),
            avs_registry_chain_reader,
            config.ws_rpc_url(),
        )
        .await;
        let token = tokio_util::sync::CancellationToken::new().clone();
        let avs_registry_service_chaincaller =
            AvsRegistryServiceChainCaller::new(avs_reader, operators_info_service.clone());
        let provider = get_ws_provider(config.ws_rpc_url().as_str()).await.unwrap();

        tokio::spawn(async move {
            let _ = operators_info_service
                .start_service(&token, 0, provider.get_block_number().await.unwrap())
                .await;
        });

        let bls_aggregation_service = BlsAggregatorService::new(avs_registry_service_chaincaller);

        Self {
            port_address: config.aggregator_ip_addr(),
            tasks_responses: HashMap::new(),
            tasks: HashMap::new(),
            bls_aggregation_service,
        }
    }

    /// Starts the aggregator service
    pub async fn start(self, ws_rpc_url: String) -> eyre::Result<()> {
        info!("Starting aggregator");

        let aggregator = Arc::new(tokio::sync::Mutex::new(self));

        // Spawn two tasks: one for the server and one for processing tasks
        let server_handle = tokio::spawn(Self::start_server(Arc::clone(&aggregator)));
        let process_handle = tokio::spawn(Self::process_tasks(
            ws_rpc_url.clone(),
            Arc::clone(&aggregator),
        ));

        // Wait for both tasks to complete and handle potential errors
        match tokio::try_join!(server_handle, process_handle) {
            Ok((server_result, process_result)) => {
                server_result?;
                process_result?;
            }
            Err(e) => {
                eprintln!("Error in task execution: {:?}", e);
                return Err(eyre::eyre!("Task execution failed"));
            }
        }

        Ok(())
    }

    /// Starts the RPC server
    ///
    /// # Arguments
    ///
    /// * `aggregator` - The aggregator
    ///
    /// # Returns
    ///
    /// * `eyre::Result<()>` - The result of the operation
    pub async fn start_server(aggregator: Arc<tokio::sync::Mutex<Self>>) -> eyre::Result<()> {
        let mut io = IoHandler::new();
        io.add_method("process_signed_task_response", {
            let aggregator = Arc::clone(&aggregator);
            move |params: Params| {
                let aggregator = Arc::clone(&aggregator);
                async move {
                    let signed_task_response: crate::rpc_server::SignedTaskResponse = match params {
                        Params::Map(map) => serde_json::from_value(map["params"].clone()).unwrap(),
                        _ => return Err(Error::invalid_params("Expected a map")),
                    };

                    // Call the process_signed_task_response function
                    let result = aggregator
                        .lock()
                        .await
                        .process_signed_task_response(signed_task_response)
                        .await;
                    match result {
                        Ok(_) => Ok(Value::Bool(true)),
                        Err(_) => Err(Error::invalid_params("invalid")),
                    }
                }
            }
        });
        let socket: SocketAddr = aggregator
            .lock()
            .await
            .port_address
            .parse()
            .expect("Unable to parse socket address");
        let server = ServerBuilder::new(io)
            .cors(DomainsValidation::AllowOnly(vec![
                AccessControlAllowOrigin::Any,
            ]))
            .start_http(&socket)
            .expect("Unable to start RPC server");

        info!("Server running at {}", socket);

        server.wait();

        Ok(())
    }

    /// Processes the tasks
    ///
    /// # Arguments
    ///
    /// * `ws_rpc_url` - The websocket RPC URL
    /// * `aggregator` - The aggregator
    ///
    /// # Returns
    ///
    /// * `eyre::Result<()>` - The result of the operation
    pub async fn process_tasks(
        ws_rpc_url: String,
        aggregator: Arc<tokio::sync::Mutex<Self>>,
    ) -> eyre::Result<()> {
        let ws = WsConnect::new(ws_rpc_url.clone());
        let provider = ProviderBuilder::new().on_ws(ws).await?;

        let filter = Filter::new().event_signature(NewTaskCreated::SIGNATURE_HASH);
        let sub = provider.subscribe_logs(&filter).await?;
        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            info!("received new task created in aggregator ");

            let NewTaskCreated { taskIndex, task } = log.log_decode()?.inner.data;

            aggregator
                .lock()
                .await
                .tasks
                .insert(taskIndex, task.clone());

            let mut quorum_nums: Vec<u8> = vec![];
            let mut quorum_threshold_percentages = Vec::with_capacity(task.quorumNumbers.len());
            for _ in &task.quorumNumbers {
                quorum_threshold_percentages
                    .push(task.quorumThresholdPercentage.try_into().unwrap());
            }

            for val in task.quorumNumbers.iter() {
                quorum_nums.push(*val);
            }

            let time_to_expiry = tokio::time::Duration::from_secs(
                (TASK_CHALLENGE_WINDOW_BLOCK * BLOCK_TIME_SECONDS).into(),
            );
            info!("initializing new task in bls aggregation service");

            let _ = aggregator
                .lock()
                .await
                .bls_aggregation_service
                .initialize_new_task(
                    taskIndex,
                    task.taskCreatedBlock,
                    quorum_nums.clone(),
                    quorum_threshold_percentages.clone(),
                    time_to_expiry,
                )
                .await
                .map_err(|e: BlsAggregationServiceError| eyre::eyre!(e));

            info!("initialized new task in bls aggregation service");
        }

        Ok(())
    }

    /// Processes the signed task response
    ///
    /// # Arguments
    ///
    /// * [`SignedTaskResponse`] - The signed task response
    ///
    /// # Returns
    ///
    /// * `eyre::Result<()>` - The result of the operation
    pub async fn process_signed_task_response(
        &mut self,
        _signed_task_response: crate::rpc_server::SignedTaskResponse,
    ) -> Result<bool, AggregatorError> {
        // let task_index = signed_task_response.task_response.referenceTaskIndex;

        // let task_response_digest =
        //     alloy::primitives::keccak256(IncredibleSquaringTaskManager::TaskResponse::abi_encode(
        //         &signed_task_response.task_response,
        //     ));

        // let response =
        //     check_double_mapping(&self.tasks_responses, task_index, task_response_digest);

        // if response.is_none() {
        //     let mut inner_map = HashMap::new();
        //     inner_map.insert(
        //         task_response_digest,
        //         signed_task_response.clone().task_response,
        //     );
        //     self.tasks_responses.insert(task_index, inner_map);
        // }

        // self.bls_aggregation_service
        //     .process_new_signature(
        //         task_index,
        //         task_response_digest,
        //         signed_task_response.signature(),
        //         signed_task_response.operator_id(),
        //     )
        //     .await
        //     .unwrap();
        // info!("processed signature for index {:?}", task_index);

        // if let Some(aggregated_response) = self
        //     .bls_aggregation_service
        //     .aggregated_response_receiver
        //     .lock()
        //     .await
        //     .recv()
        //     .await
        // {
        //     let res = self.send_aggregated_response_to_contract(aggregated_response.unwrap())
        //         .await;
        //     return Ok(res);
        // }
        Ok(true)
    }

    /// Sends the aggregated response to the contract
    ///
    /// # Arguments
    ///
    /// * [`BlsAggregationServiceResponse`] - The aggregated response
    ///
    /// # Returns
    ///
    /// * `eyre::Result<()>` - The result of the operation
    pub async fn send_aggregated_response_to_contract(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> bool {
        let mut non_signer_pub_keys = Vec::<G1Point>::new();
        for pub_key in response.non_signers_pub_keys_g1.iter() {
            let g1 = convert_to_g1_point(pub_key.g1()).unwrap();
            non_signer_pub_keys.push(G1Point { X: g1.X, Y: g1.Y })
        }

        let mut quorum_apks = Vec::<G1Point>::new();
        for pub_key in response.quorum_apks_g1.iter() {
            let g1 = convert_to_g1_point(pub_key.g1()).unwrap();
            quorum_apks.push(G1Point { X: g1.X, Y: g1.Y })
        }

        let _non_signer_stakes_and_signature = NonSignerStakesAndSignature {
            nonSignerPubkeys: non_signer_pub_keys,
            nonSignerQuorumBitmapIndices: response.non_signer_quorum_bitmap_indices,
            quorumApks: quorum_apks,
            apkG2: G2Point {
                X: convert_to_g2_point(response.signers_apk_g2.g2()).unwrap().X,
                Y: convert_to_g2_point(response.signers_apk_g2.g2()).unwrap().Y,
            },
            sigma: G1Point {
                X: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())
                    .unwrap()
                    .X,
                Y: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())
                    .unwrap()
                    .Y,
            },
            quorumApkIndices: response.quorum_apk_indices,
            totalStakeIndices: response.total_stake_indices,
            nonSignerStakeIndices: response.non_signer_stake_indices,
        };

        let _task = &self.tasks[&response.task_index];
        let _task_response =
            &self.tasks_responses[&response.task_index][&response.task_response_digest];
        true // don't actually send the call, return true
    }
}

#[cfg(test)]
mod tests {

    use alloy::primitives::{FixedBytes, U256};
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_types::test::TestOperator;
    use incredible_testing_utils::{
        get_incredible_squaring_operator_state_retriever,
        get_incredible_squaring_registry_coordinator,
    };
    use std::time::Duration;

    use super::*;
    const INCREDIBLE_CONFIG_FILE: &str = r#"
    [rpc_config]
    chain_id = 31337
    http_rpc_url = "http://localhost:8545"
    ws_rpc_url = "ws://localhost:8545"
    signer = "0x337edbf6fef9af147f49c04c1004da47a8bf9f88c01022b7dd108e31c037f075"

    [ecdsa_config]
    keystore_path = "../testing-utils/src/ecdsakeystore.json"
    keystore_password = "test"

    [bls_config]
    keystore_path = "../testing-utils/src/blskeystore.json"
    keystore_password = "testpassword"

    [operator_config]
    operator_address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
    operator_id = "0xb345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3"

    [aggregator_config]
    ip_address = "127.0.0.1:8080"

    "#;

    const PRIVATE_KEY_DECIMAL: &str =
        "12248929636257230549931416853095037629726205319386239410403476017439825112537";
    const OPERATOR_ID: &str = "b345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3";
    #[allow(unused)]
    fn build_test_operator() -> TestOperator {
        let bls_keypair = BlsKeyPair::new(PRIVATE_KEY_DECIMAL.into()).unwrap();
        let operator_id =
            FixedBytes::<32>::from_slice(hex::decode(OPERATOR_ID).unwrap().as_slice());
        TestOperator {
            operator_id,
            bls_keypair: bls_keypair.clone(),
            stake_per_quorum: HashMap::from([(1u8, U256::from(123))]),
        }
    }

    async fn build_aggregator() -> FakeAggregator {
        let mut incredible_config: IncredibleConfig =
            toml::from_str(INCREDIBLE_CONFIG_FILE).unwrap();
        incredible_config.set_registry_coordinator_addr(
            get_incredible_squaring_registry_coordinator()
                .await
                .to_string(),
        );
        incredible_config.set_operator_state_retriever(
            get_incredible_squaring_operator_state_retriever()
                .await
                .to_string(),
        );
        FakeAggregator::new(incredible_config).await
    }

    #[tokio::test]
    async fn test_build() {
        let fake_aggregator = build_aggregator().await;
        fake_aggregator
            .bls_aggregation_service
            .initialize_new_task(
                0,
                5,
                ["0".parse().unwrap()].to_vec(),
                ["100".parse().unwrap()].to_vec(),
                Duration::from_secs(1200),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_start_server() {
        let fake_aggregator = build_aggregator().await;
        let _ = tokio::spawn(async move {
            let _ =
                FakeAggregator::start_server(Arc::new(tokio::sync::Mutex::new(fake_aggregator)))
                    .await;
        });
    }
}
