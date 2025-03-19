//! integration tests for incredible squaring

#[cfg(test)]
mod tests {
    use alloy::primitives::{FixedBytes, U256};
    use eigensdk::common::get_provider;
    use eigensdk::crypto_bls::BlsKeyPair;
    use eigensdk::logging::{init_logger, log_level::LogLevel};
    use eigensdk::testing_utils::anvil_constants::{
        get_allocation_manager_address, get_avs_directory_address, get_delegation_manager_address,
        get_erc20_mock_strategy, get_permission_controller_address,
        get_rewards_coordinator_address, get_strategy_manager_address,
    };
    use incredible_aggregator::Aggregator;
    use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager;
    use incredible_challenger::Challenger;
    use incredible_config::IncredibleConfig;
    use incredible_operator::builder::OperatorBuilder;
    use incredible_operator_2::builder::OperatorBuilder as Operator2Builder;
    use incredible_squaring_avs::commands::avs::{
        create_total_delegated_stake_quorum, modify_allocation_for_operator,
        register_for_operator_sets, register_operator_with_el_and_deposit_tokens_in_strategy,
    };
    use incredible_testing_utils::{
        get_incredible_squaring_operator_state_retriever,
        get_incredible_squaring_registry_coordinator, get_incredible_squaring_service_manager,
        get_incredible_squaring_strategy_address, get_incredible_squaring_task_manager,
    };
    use rust_bls_bn254::keystores::base_keystore::Keystore;
    use std::{
        sync::Arc,
        time::{SystemTime, UNIX_EPOCH},
    };
    const ANVIL_HTTP_URL: &str = "http://localhost:8545";

    const INCREDIBLE_CONFIG_FILE: &str = r#"
    [rpc_config]
    chain_id = 31337
    http_rpc_url = "http://localhost:8545"
    ws_rpc_url = "ws://localhost:8545"
    signer = "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6"

    [ecdsa_config]
    keystore_path = "../crates/testing-utils/src/ecdsakeystore.json"
    keystore_password = "test"
    keystore_2_path = "../crates/testing-utils/src/ecdsa_keystore_2.json"
    keystore_2_password = "test"

    [bls_config]
    keystore_path = "../crates/testing-utils/src/blskeystore.json"
    keystore_password = "testpassword"
    keystore_2_path = "../crates/testing-utils/src/bls_keystore_2.json"
    keystore_2_password = "test"

    [operator_config]
    operator_address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
    operator_id = "0xb345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3"
    operator_2_address = "0x0b065a0423f076a340f37e16e1ce22e23d66caf2"
    operator_2_id = "0x17a0935b43b64cc3536d48621208fddb680ef8998561f0a1669a3ccda66676be"
    operator_set_id = "0"
    operator_1_token_amount = "5000000000000000000000"
    operator_2_token_amount = "7000000000000000000000"
    allocation_delay = "0"
    operator_1_times_failing = "100"
    operator_2_times_failing = "100"
    [aggregator_config]
    ip_address = "127.0.0.1:8080"

    [operator_registration_config]
    register_operator = true
    operator_pvt_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
    operator_to_avs_registration_sig_salt = "b345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3"
    socket = "incredible"
    quorum_number = "00"
    sig_expiry = "10"

    [operator_2_registration_config]
    operator_pvt_key = "0x9385907a38014b53604fd848bf907453f3b4f774db8ffa72b9960f06b238eb15"
    register_operator = true
    operator_to_avs_registration_sig_salt = "b345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3"
    socket = "incredible"
    quorum_number = "00"
    sig_expiry = "10"
    "#;

    async fn register_operator_with_el_and_avs(
        operator_pvt_key: Option<String>,
        ecdsa_keystore_path: String,
        ecdsa_keystore_password: String,
        operator_token_amount: U256,
    ) {
        let mut incredible_config: IncredibleConfig =
            toml::from_str(INCREDIBLE_CONFIG_FILE).unwrap();

        incredible_config.set_registry_coordinator_addr(
            get_incredible_squaring_registry_coordinator()
                .await
                .to_string(),
        );
        incredible_config.set_allocation_manager_address(
            get_allocation_manager_address(ANVIL_HTTP_URL.to_string())
                .await
                .to_string(),
        );
        incredible_config.set_operator_state_retriever(
            get_incredible_squaring_operator_state_retriever()
                .await
                .to_string(),
        );
        let delegation_manager_address_anvil =
            get_delegation_manager_address(ANVIL_HTTP_URL.to_string()).await;
        let avs_directory_address_anvil =
            get_avs_directory_address(ANVIL_HTTP_URL.to_string()).await;

        let strategy_manager_address_anvil =
            get_strategy_manager_address(ANVIL_HTTP_URL.to_string()).await;
        let erc20_mock_strategy_address_anvil = get_incredible_squaring_strategy_address().await;

        incredible_config.set_delegation_manager_addr(delegation_manager_address_anvil.to_string());
        incredible_config.set_avs_directory_address(avs_directory_address_anvil.to_string());
        incredible_config.set_strategy_manager_addr(strategy_manager_address_anvil.to_string());
        incredible_config
            .set_erc20_mock_strategy_address(erc20_mock_strategy_address_anvil.to_string());

        let now = SystemTime::now();
        let mut expiry: U256 = U256::from(0);
        if let Ok(duration_since_epoch) = now.duration_since(UNIX_EPOCH) {
            let seconds = duration_since_epoch.as_secs(); // Returns a u64

            // Signature expiry is at 10000 seconds
            expiry = U256::from(seconds) + U256::from(10000);
        } else {
            println!("System time seems to be before the UNIX epoch.");
        }

        incredible_config.set_sig_expiry(expiry.to_string());
        register_operator_with_el_and_deposit_tokens_in_strategy(
            "metadata".to_string(),
            0, // allocation delay
            operator_pvt_key,
            incredible_config.http_rpc_url(),
            ecdsa_keystore_path,
            ecdsa_keystore_password,
            get_permission_controller_address(ANVIL_HTTP_URL.to_string()).await,
            get_rewards_coordinator_address(ANVIL_HTTP_URL.to_string()).await,
            get_allocation_manager_address(ANVIL_HTTP_URL.to_string()).await,
            incredible_config.registry_coordinator_addr().unwrap(),
            incredible_config.delegation_manager_addr().unwrap(),
            incredible_config.avs_directory_addr().unwrap(),
            incredible_config.strategy_manager_addr().unwrap(),
            incredible_config.erc20_mock_strategy_addr().unwrap(),
            operator_token_amount,
        )
        .await
        .unwrap();
    }

    async fn test_incredible_squaring_without_challenger() {
        init_logger(LogLevel::Info);
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let mut incredible_config: IncredibleConfig =
            toml::from_str(INCREDIBLE_CONFIG_FILE).unwrap();
        incredible_config.set_aggregator_ip_address("127.0.0.1:8081".to_string());
        incredible_config.set_registry_coordinator_addr(
            get_incredible_squaring_registry_coordinator()
                .await
                .to_string(),
        );
        incredible_config.set_permission_controller_address(
            get_permission_controller_address(ANVIL_HTTP_URL.to_string())
                .await
                .to_string(),
        );
        incredible_config.set_allocation_manager_address(
            get_allocation_manager_address(ANVIL_HTTP_URL.to_string())
                .await
                .to_string(),
        );
        incredible_config.set_service_manager_address(
            get_incredible_squaring_service_manager().await.to_string(),
        );
        incredible_config.set_operator_state_retriever(
            get_incredible_squaring_operator_state_retriever()
                .await
                .to_string(),
        );

        create_total_delegated_stake_quorum(
            get_incredible_squaring_strategy_address().await,
            incredible_config.registry_coordinator_addr().unwrap(),
            incredible_config.operator_pvt_key(),
            incredible_config.ecdsa_keystore_path(),
            incredible_config.ecdsa_keystore_password(),
            ANVIL_HTTP_URL,
        )
        .await
        .unwrap();
        register_operator_with_el_and_avs(
            incredible_config.operator_pvt_key(),
            incredible_config.ecdsa_keystore_path(),
            incredible_config.ecdsa_keystore_password(),
            incredible_config.operator_1_token_amount().unwrap(),
        )
        .await;
        modify_allocation_for_operator(
            incredible_config.operator_set_id().unwrap(),
            get_allocation_manager_address(ANVIL_HTTP_URL.to_string()).await,
            incredible_config.operator_pvt_key(),
            incredible_config.ecdsa_keystore_path(),
            incredible_config.ecdsa_keystore_password(),
            ANVIL_HTTP_URL,
            incredible_config.service_manager_addr().unwrap(),
            [get_incredible_squaring_strategy_address().await].to_vec(),
            [100].to_vec(),
        )
        .await
        .unwrap();

        let keystore = Keystore::from_file(&incredible_config.bls_keystore_path())
            .unwrap()
            .decrypt(&incredible_config.bls_keystore_password())
            .unwrap();
        let fr_key: String = keystore.iter().map(|&value| value as char).collect();
        let key_pair = BlsKeyPair::new(fr_key).unwrap();
        register_for_operator_sets(
            incredible_config.operator_set_id().unwrap(),
            key_pair,
            incredible_config.permission_controller_address().unwrap(),
            incredible_config.registry_coordinator_addr().unwrap(),
            get_allocation_manager_address(ANVIL_HTTP_URL.to_string()).await,
            incredible_config.operator_pvt_key(),
            incredible_config.ecdsa_keystore_path(),
            incredible_config.ecdsa_keystore_password(),
            ANVIL_HTTP_URL,
            incredible_config.service_manager_addr().unwrap(),
            incredible_config.socket().to_string(),
        )
        .await
        .unwrap();

        let mut operator_builder = OperatorBuilder::build(incredible_config.clone())
            .await
            .unwrap();

        tokio::spawn(async move {
            operator_builder.start_operator().await.unwrap();
        });

        let (aggregator, aggregate_receiver) =
            Aggregator::new(incredible_config.clone()).await.unwrap();

        let arc_agg = Arc::new(tokio::sync::Mutex::new(aggregator));
        let arc_agg_clone = Arc::clone(&arc_agg);
        let arc_agg_for_server = Arc::clone(&arc_agg);

        // Run process_tasks in a separate thread
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                if let Err(e) =
                    Aggregator::process_tasks("ws://localhost:8545".to_string(), arc_agg_clone)
                        .await
                {
                    eprintln!("Process tasks error: {:?}", e);
                }
            });
        });

        // Run the server in a separate thread
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                if let Err(e) = Aggregator::start_server(Arc::clone(&arc_agg_for_server)).await {
                    eprintln!("Server error: {:?}", e);
                }
            });
        });

        tokio::spawn(async move {
            Aggregator::process_aggregator_responses(Arc::clone(&arc_agg), aggregate_receiver)
                .await
                .unwrap();
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let task_generator = incredible_task_generator::TaskManager::new(
            get_incredible_squaring_task_manager().await,
            "http://localhost:8545".to_string(),
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d".to_string(),
            incredible_config.quorum_number().unwrap().to_string(),
        );
        task_generator
            .create_new_task("2".parse().unwrap())
            .await
            .unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        let task_manager_contract = IncredibleSquaringTaskManager::new(
            get_incredible_squaring_task_manager().await,
            get_provider("http://localhost:8545"),
        );
        let latest_task_num = task_manager_contract
            .latestTaskNum()
            .call()
            .await
            .unwrap()
            ._0;

        let task_hash = task_manager_contract
            .allTaskHashes(latest_task_num - 1)
            .call()
            .await
            .unwrap()
            ._0;
        assert_ne!(FixedBytes::<32>::default(), task_hash);

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        let response_hash = task_manager_contract
            .allTaskResponses(latest_task_num - 1)
            .call()
            .await
            .unwrap()
            ._0;
        assert_ne!(FixedBytes::<32>::default(), response_hash);

        let is_challenge_success = task_manager_contract
            .taskSuccesfullyChallenged(latest_task_num - 1)
            .call()
            .await
            .unwrap()
            ._0;

        assert!(!is_challenge_success);
    }

    async fn test_incredible_squaring_with_challenger() {
        init_logger(LogLevel::Info);
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let mut incredible_config: IncredibleConfig =
            toml::from_str(INCREDIBLE_CONFIG_FILE).unwrap();
        incredible_config.set_aggregator_ip_address("127.0.0.1:8081".to_string());
        incredible_config.set_registry_coordinator_addr(
            get_incredible_squaring_registry_coordinator()
                .await
                .to_string(),
        );
        incredible_config.set_delegation_manager_addr(
            get_delegation_manager_address(ANVIL_HTTP_URL.to_string())
                .await
                .to_string(),
        );
        incredible_config.set_erc20_mock_strategy_address(
            get_erc20_mock_strategy(ANVIL_HTTP_URL.to_string())
                .await
                .to_string(),
        );
        incredible_config.set_permission_controller_address(
            get_permission_controller_address(ANVIL_HTTP_URL.to_string())
                .await
                .to_string(),
        );
        incredible_config.set_allocation_manager_address(
            get_allocation_manager_address(ANVIL_HTTP_URL.to_string())
                .await
                .to_string(),
        );
        incredible_config.set_service_manager_address(
            get_incredible_squaring_service_manager().await.to_string(),
        );
        incredible_config.set_operator_state_retriever(
            get_incredible_squaring_operator_state_retriever()
                .await
                .to_string(),
        );

        create_total_delegated_stake_quorum(
            get_incredible_squaring_strategy_address().await,
            incredible_config.registry_coordinator_addr().unwrap(),
            incredible_config.operator_pvt_key(),
            incredible_config.ecdsa_keystore_path(),
            incredible_config.ecdsa_keystore_password(),
            ANVIL_HTTP_URL,
        )
        .await
        .unwrap();

        register_operator_with_el_and_avs(
            incredible_config.operator_2_pvt_key(),
            incredible_config.ecdsa_keystore_2_path(),
            incredible_config.ecdsa_keystore_2_password(),
            incredible_config.operator_2_token_amount().unwrap(),
        )
        .await;

        modify_allocation_for_operator(
            incredible_config.operator_set_id().unwrap(),
            get_allocation_manager_address(ANVIL_HTTP_URL.to_string()).await,
            incredible_config.operator_2_pvt_key(),
            incredible_config.ecdsa_keystore_2_path(),
            incredible_config.ecdsa_keystore_2_password(),
            ANVIL_HTTP_URL,
            incredible_config.service_manager_addr().unwrap(),
            [get_incredible_squaring_strategy_address().await].to_vec(),
            [100].to_vec(),
        )
        .await
        .unwrap();

        let keystore = Keystore::from_file(&incredible_config.bls_keystore_2_path())
            .unwrap()
            .decrypt(&incredible_config.bls_keystore_2_password())
            .unwrap();
        let fr_key: String = keystore.iter().map(|&value| value as char).collect();
        let key_pair = BlsKeyPair::new(fr_key).unwrap();
        register_for_operator_sets(
            incredible_config.operator_set_id().unwrap(),
            key_pair,
            incredible_config.permission_controller_address().unwrap(),
            incredible_config.registry_coordinator_addr().unwrap(),
            get_allocation_manager_address(ANVIL_HTTP_URL.to_string()).await,
            incredible_config.operator_2_pvt_key(),
            incredible_config.ecdsa_keystore_2_path(),
            incredible_config.ecdsa_keystore_2_password(),
            ANVIL_HTTP_URL,
            incredible_config.service_manager_addr().unwrap(),
            incredible_config.socket().to_string(),
        )
        .await
        .unwrap();

        let op_builder = OperatorBuilder::build(incredible_config.clone())
            .await
            .unwrap();
        let client = Some(Arc::new(op_builder.client.clone()));
        let mut operator_builder = Operator2Builder::build(incredible_config.clone(), client)
            .await
            .unwrap();

        tokio::spawn(async move {
            operator_builder.start_operator().await.unwrap();
        });

        let (aggregator, aggregate_receiver) =
            Aggregator::new(incredible_config.clone()).await.unwrap();

        let arc_agg = Arc::new(tokio::sync::Mutex::new(aggregator));
        let arc_agg_clone = Arc::clone(&arc_agg);
        let arc_agg_for_server = Arc::clone(&arc_agg);

        // Run process_tasks in a separate thread
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                if let Err(e) =
                    Aggregator::process_tasks("ws://localhost:8545".to_string(), arc_agg_clone)
                        .await
                {
                    eprintln!("Process tasks error: {:?}", e);
                }
            });
        });

        // Run the server in a separate thread
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                if let Err(e) = Aggregator::start_server(Arc::clone(&arc_agg_for_server)).await {
                    eprintln!("Server error: {:?}", e);
                }
            });
        });

        tokio::spawn(async move {
            Aggregator::process_aggregator_responses(Arc::clone(&arc_agg), aggregate_receiver)
                .await
                .unwrap();
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let mut challenger = Challenger::build(incredible_config.clone()).await.unwrap();
        tokio::spawn(async move {
            challenger.start_challenger().await.unwrap();
        });

        let task_generator = incredible_task_generator::TaskManager::new(
            get_incredible_squaring_task_manager().await,
            "http://localhost:8545".to_string(),
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d".to_string(),
            incredible_config.quorum_number().unwrap().to_string(),
        );
        task_generator
            .create_new_task("2".parse().unwrap())
            .await
            .unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        let task_manager_contract = IncredibleSquaringTaskManager::new(
            get_incredible_squaring_task_manager().await,
            get_provider("http://localhost:8545"),
        );
        let latest_task_num = task_manager_contract
            .latestTaskNum()
            .call()
            .await
            .unwrap()
            ._0;

        let task_hash = task_manager_contract
            .allTaskHashes(latest_task_num - 1)
            .call()
            .await
            .unwrap()
            ._0;
        assert_ne!(FixedBytes::<32>::default(), task_hash);

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        let response_hash = task_manager_contract
            .allTaskResponses(latest_task_num - 1)
            .call()
            .await
            .unwrap()
            ._0;
        assert_ne!(FixedBytes::<32>::default(), response_hash);

        let is_challenge_success = task_manager_contract
            .taskSuccesfullyChallenged(latest_task_num - 1)
            .call()
            .await
            .unwrap()
            ._0;

        assert!(is_challenge_success);
    }

    #[tokio::test]
    async fn run_tests_in_order() {
        test_incredible_squaring_without_challenger().await;
        test_incredible_squaring_with_challenger().await;
    }
}
