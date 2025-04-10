use alloy::hex;
use alloy::primitives::aliases::U96;
use alloy::primitives::{Address, FixedBytes, U256};
use alloy::providers::Provider;
use alloy::signers::local::{LocalSigner, PrivateKeySigner};
use clap::value_parser;
use clap::{Args, Parser};
use eigensdk::client_elcontracts::reader::ELChainReader;
use eigensdk::client_elcontracts::{error::ElContractsError, writer::ELChainWriter};
use eigensdk::common::{get_provider, get_signer};
use eigensdk::crypto_bls::BlsKeyPair;
use eigensdk::logging::{get_logger, init_logger, log_level::LogLevel};
use eigensdk::metrics::prometheus::init_registry;
use eigensdk::types::operator::Operator;
use eigensdk::utils::slashing::core::allocationmanager::AllocationManager::{self, OperatorSet};
use eigensdk::utils::slashing::core::allocationmanager::IAllocationManagerTypes::AllocateParams;
use eigensdk::utils::slashing::middleware::registrycoordinator::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
use eigensdk::utils::slashing::middleware::registrycoordinator::IStakeRegistryTypes::StrategyParams;
use eigensdk::utils::slashing::middleware::registrycoordinator::RegistryCoordinator;
use incredible_avs::builder::{AvsBuilder, DefaultAvsLauncher, LaunchAvs};
use incredible_config::IncredibleConfig;
use rust_bls_bn254::keystores::base_keystore::Keystore;
use std::ffi::OsString;
use std::fmt;
use std::net::SocketAddr;
use std::process::{Command, Stdio};
use std::str::FromStr;
use tracing::info;

/// No Additional arguments
#[derive(Debug, Clone, Copy, Default, Args)]
#[non_exhaustive]
pub struct NoArgs;

use std::path::PathBuf;

/// Starts incredible squaring
#[derive(Debug, Parser)]
pub struct AvsCommand<Ext: Args + fmt::Debug = NoArgs> {
    /// The EVM chain ID.
    #[arg(
    long,
    value_name = "CHAIN_ID",
    global = true,
    default_value_t = 31337,
    value_parser = value_parser!(u16).range(1..)
)]
    chain_id: u16,

    /// The RPC URL of the node.
    #[arg(long, value_name = "RPC_URL",default_value = "http://localhost:8545", value_parser = clap::value_parser!(String))]
    rpc_url: String,

    /// The RPC URL of the node.
    #[arg(long, value_name = "WS_RPC_URL",default_value = "ws://localhost:8545", value_parser = clap::value_parser!(String))]
    ws_rpc_url: String,

    /// ECDSA key store path file
    #[arg(
        long,
        value_name = "ECDSA_KEYSTORE_PATH",
        default_value = "./crates/testing-utils/src/ecdsakeystore.json"
    )]
    ecdsa_keystore_path: String,

    /// ECDSA keystore path  password
    #[arg(long, value_name = "ECDSA_KEYSTORE_PASSWORD", default_value = "test")]
    ecdsa_keystore_password: String,

    /// ECDSA key store path file
    #[arg(
        long,
        value_name = "ECDSA_KEYSTORE_2_PATH",
        default_value = "./crates/testing-utils/src/ecdsa_keystore_2.json"
    )]
    ecdsa_keystore_2_path: String,

    /// ECDSA keystore path  password
    #[arg(long, value_name = "ECDSA_KEYSTORE_2_PASSWORD", default_value = "test")]
    ecdsa_keystore_2_password: String,

    /// Registry coordinator address
    #[arg(long, value_name = "REGISTRY_COORDINATOR_ADDR")]
    registry_coordinator_address: Option<String>,

    /// Delegation Manager address
    #[arg(long, value_name = "DELEGATION_MANAGER_ADDRESS")]
    delegation_manager_address: Option<String>,

    /// Aggregator Ip address
    #[arg(
        long,
        value_name = "AGGREGATOR_IP_ADDRESS",
        default_value = "127.0.0.1:8080"
    )]
    aggregator_ip_address: String,

    /// Metrics port address
    #[arg(long, value_name = "METRICS_ADDRESS", default_value = "127.0.0.1:9001")]
    metrics_address: String,

    /// bls keystore path
    #[arg(
        long,
        value_name = "BLS_KEYSTORE_PATH",
        default_value = "./crates/testing-utils/src/blskeystore.json"
    )]
    bls_keystore_path: String,

    /// bls keystore password
    #[arg(
        long,
        value_name = "BLS_KEYSTORE_PASSWORD",
        default_value = "testpassword"
    )]
    bls_keystore_password: String,

    /// bls keystore path
    #[arg(
        long,
        value_name = "BLS_KEYSTORE_2_PATH",
        default_value = "./crates/testing-utils/src/bls_keystore_2.json"
    )]
    bls_keystore_2_path: String,

    /// bls keystore password
    #[arg(long, value_name = "BLS_KEYSTORE_2_PASSWORD", default_value = "test")]
    bls_keystore_2_password: String,

    /// operator set id
    #[arg(long, value_name = "OPERATOR_SET_ID", default_value = "0")]
    operator_set_id: String,

    /// Operator Id
    #[arg(
        long,
        value_name = "OPERATOR_ID",
        default_value = "0xb345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3"
    )]
    operator_id: String,

    /// Operator2 Id
    #[arg(
        long,
        value_name = "OPERATOR_2_ID",
        default_value = "0x17a0935b43b64cc3536d48621208fddb680ef8998561f0a1669a3ccda66676be"
    )]
    operator_2_id: String,

    /// Allocation Delay , default 0
    #[arg(long, value_name = "ALLOCATION_DELAY", default_value = "0")]
    allocation_delay: String,

    /// Operator State retreiver
    #[arg(long, value_name = "OPERATOR_STATE_RETRIEVER_ADDRESS")]
    operator_state_retriever_addr: Option<String>,

    /// Avs Directory
    #[arg(long, value_name = "AVS_DIRECTORY_ADDRESS")]
    avs_directory_addr: Option<String>,

    /// Strategy Manager
    #[arg(long, value_name = "STRATEGY_MANAGER_ADDRESS")]
    strategy_manager_addr: Option<String>,

    /// Operator Address
    #[arg(
        long,
        value_name = "OPERATOR_ADDRESS",
        default_value = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
    )]
    operator_address: String,

    /// Operator2 Address
    #[arg(
        long,
        value_name = "OPERATOR_2_ADDRESS",
        default_value = "0x0b065a0423f076a340f37e16e1ce22e23d66caf2"
    )]
    operator_2_address: String,

    /// Allocation Manager address
    #[arg(long, value_name = "ALLOCATION_MANAGER_ADDRESS")]
    allocation_manager_address: Option<String>,

    #[arg(long, value_name = "REGISTER_OPERATOR", default_value = "true")]
    register_operator: bool,

    #[arg(
        long,
        value_name = "OPERATOR_TO_AVS_REGISTRATION_SIG_SALT",
        default_value = "b345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3"
    )]
    operator_to_avs_registration_sig_salt: String,

    #[arg(
        long,
        value_name = "OPERATOR_TO_AVS_REGISTRATION_SIG_SALT",
        default_value = "b345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3"
    )]
    operator_2_to_avs_registration_sig_salt: String,

    #[arg(long, value_name = "SOCKET", default_value = "incredible-socket")]
    socket: String,

    #[arg(long, value_name = "QUORUM_NUMBER", default_value = "00")]
    quorum_number: String,

    #[arg(long, value_name = "SIG_EXPIRY")]
    sig_expiry: Option<String>,

    #[arg(long, value_name = "TASK_MANAGER")]
    task_manager_addr: Option<String>,

    #[arg(
        long,
        value_name = "SIGNER",
        default_value = "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6"
    )]
    signer: String,

    #[arg(long, value_name = "ERC20_MOCK_STRATEGY_ADDRESS")]
    erc20_mock_strategy_address: Option<String>,

    // metadata uri
    #[arg(
        long,
        value_name = "METADATA_URI",
        default_value = "incredible-squaring-avs"
    )]
    metadata_uri: String,

    // default is no.2 of anvil
    #[arg(
        long,
        value_name = "TASK_MANAGER_SIGNER",
        default_value = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d"
    )]
    task_manager_signer: String,

    #[arg(
        long,
        value_name = "OPERATOR_PVT_KEY",
        default_value = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
    )]
    operator_pvt_key: String,

    #[arg(long, value_name = "TIMES_FAILING_OPERATOR_1", default_value = "90")]
    times_failing_operator_1: String,

    #[arg(long, value_name = "TIMES_FAILING_OPERATOR_2", default_value = "10")]
    times_failing_operator_2: String,

    #[arg(
        long,
        value_name = "OPERATOR_2_PVT_KEY",
        default_value = "0x9385907a38014b53604fd848bf907453f3b4f774db8ffa72b9960f06b238eb15"
    )]
    operator_2_pvt_key: String,

    #[arg(long, value_name = "OPERATOR_2_SIG_EXPIRY")]
    operator_2_sig_expiry: Option<String>,

    #[arg(
        long,
        value_name = "OPERATOR_1_TOKEN_AMOUNT",
        default_value = "5000000000000000000000"
    )]
    operator_1_token_amount: String,

    #[arg(
        long,
        value_name = "OPERATOR_2_TOKEN_AMOUNT",
        default_value = "7000000000000000000000"
    )]
    operator_2_token_amount: String,

    #[arg(
        long,
        value_name = "NODE_API_ADDRESS",
        default_value = "127.0.0.1:8085"
    )]
    node_api_address: String,

    /// The path to the configuration file to use.
    #[arg(long, value_name = "FILE")]
    config_path: Option<PathBuf>,

    /// additional arguments
    #[command(flatten, next_help_heading = "Extension")]
    pub ext: Ext,
}

impl AvsCommand {
    /// Parsers only the default CLI arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Parsers only the default [`NodeCommand`] arguments from the given iterator
    pub fn try_parse_args_from<I, T>(itr: I) -> Result<Self, clap::error::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::try_parse_from(itr)
    }
}

impl<Ext: clap::Args + fmt::Debug + Send + Sync + 'static> AvsCommand<Ext> {
    /// Execute function
    pub async fn execute(self: Box<Self>) -> eyre::Result<()> {
        init_logger(LogLevel::Info);

        let Some(config_path) = self.config_path else {
            return Err(eyre::eyre!("NO CONFIG PATH PROVIDED"));
        };

        let mut config = IncredibleConfig::load(&config_path)?;

        let socket_addr_metrics: SocketAddr = SocketAddr::from_str(&config.metrics_port_address())?;
        init_registry(socket_addr_metrics);

        let total_delegated_quorum_create_tx_hash = create_total_delegated_stake_quorum(
            config.erc20_mock_strategy_addr()?,
            config.registry_coordinator_addr()?,
            config.operator_pvt_key(),
            config.ecdsa_keystore_path(),
            config.ecdsa_keystore_password(),
            &config.http_rpc_url(),
        )
        .await?;
        info!(tx_hash = %total_delegated_quorum_create_tx_hash,"total delegated stake quorum create tx_hash");

        if self.register_operator {
            let _ = register_operator_with_el_and_deposit_tokens_in_strategy(
                self.metadata_uri.clone(),
                config.allocation_delay()?,
                config.operator_pvt_key(),
                config.http_rpc_url(),
                config.ecdsa_keystore_path(),
                config.ecdsa_keystore_password(),
                config.permission_controller_address()?,
                config.rewards_coordinator_address()?,
                config.allocation_manager_addr()?,
                config.registry_coordinator_addr()?,
                config.delegation_manager_addr()?,
                config.avs_directory_addr()?,
                config.strategy_manager_addr()?,
                config.erc20_mock_strategy_addr()?,
                config.operator_1_token_amount()?,
            )
            .await;
            let _ = register_operator_with_el_and_deposit_tokens_in_strategy(
                self.metadata_uri.clone(),
                config.allocation_delay()?,
                config.operator_2_pvt_key(),
                config.http_rpc_url(),
                config.ecdsa_keystore_2_path(),
                config.ecdsa_keystore_2_password(),
                config.permission_controller_address()?,
                config.rewards_coordinator_address()?,
                config.allocation_manager_addr()?,
                config.registry_coordinator_addr()?,
                config.delegation_manager_addr()?,
                config.avs_directory_addr()?,
                config.strategy_manager_addr()?,
                config.erc20_mock_strategy_addr()?,
                config.operator_2_token_amount()?,
            )
            .await;

            let allocation_delay_set_tx_hash = set_allocation_delay(
                config.allocation_delay()?,
                config.allocation_manager_addr()?,
                config.operator_pvt_key(),
                config.ecdsa_keystore_path(),
                config.ecdsa_keystore_password(),
                &config.http_rpc_url(),
            )
            .await?;
            info!(
                "set allocation delay for operator1 {:?}",
                allocation_delay_set_tx_hash
            );

            let modify_allocation_for_operator1_tx_hash = modify_allocation_for_operator(
                config.operator_set_id()?,
                config.allocation_manager_addr()?,
                config.operator_pvt_key(),
                config.ecdsa_keystore_path(),
                config.ecdsa_keystore_password(),
                &config.http_rpc_url(),
                config.service_manager_addr()?,
                vec![config.erc20_mock_strategy_addr()?],
                vec![1000000000000000000],
            )
            .await?;

            info!(tx_hash = %modify_allocation_for_operator1_tx_hash,strategy_address = %config.erc20_mock_strategy_addr()?,"allocation by operator1 for strategy");

            let modify_allocation_for_operator2_tx_hash = modify_allocation_for_operator(
                config.operator_set_id()?,
                config.allocation_manager_addr()?,
                config.operator_2_pvt_key(),
                config.ecdsa_keystore_2_path(),
                config.ecdsa_keystore_2_password(),
                &config.http_rpc_url(),
                config.service_manager_addr()?,
                vec![config.erc20_mock_strategy_addr()?],
                vec![1000000000000000000],
            )
            .await?;
            info!(tx_hash = %modify_allocation_for_operator2_tx_hash,strategy_address = %config.erc20_mock_strategy_addr()?,"allocation by operator2 for strategy");
            let keystore = Keystore::from_file(&config.bls_keystore_path())?
                .decrypt(&config.bls_keystore_password())
                .unwrap();
            let fr_key: String = keystore.iter().map(|&value| value as char).collect();
            let key_pair = BlsKeyPair::new(fr_key)?;
            let register_for_operator_sets_by_operator1_txhash = register_for_operator_sets(
                config.operator_set_id()?,
                key_pair,
                config.permission_controller_address()?,
                config.registry_coordinator_addr()?,
                config.allocation_manager_addr()?,
                config.operator_pvt_key(),
                config.ecdsa_keystore_path(),
                config.ecdsa_keystore_password(),
                &config.http_rpc_url(),
                config.service_manager_addr()?,
                config.socket().to_string(),
            )
            .await?;
            info!(tx_hash = %register_for_operator_sets_by_operator1_txhash,"register for operator sets by operator1");
            let keystore = Keystore::from_file(&config.bls_keystore_2_path())?
                .decrypt(&config.bls_keystore_2_password())
                .unwrap();
            let fr_key: String = keystore.iter().map(|&value| value as char).collect();
            let key_pair_2 = BlsKeyPair::new(fr_key)?;
            let register_for_operator_sets_by_operator2_txhash = register_for_operator_sets(
                config.operator_set_id()?,
                key_pair_2,
                config.permission_controller_address()?,
                config.registry_coordinator_addr()?,
                config.allocation_manager_addr()?,
                config.operator_2_pvt_key(),
                config.ecdsa_keystore_2_path(),
                config.ecdsa_keystore_2_password(),
                &config.http_rpc_url(),
                config.service_manager_addr()?,
                config.operator_2_socket().to_string(),
            )
            .await?;
            info!(tx_hash = %register_for_operator_sets_by_operator2_txhash,"register for operator sets by operator2");

            let current_block_number = get_provider(&config.http_rpc_url())
                .get_block_number()
                .await?;

            fn mine_anvil_block(rpc_url: &str, blocks: u64) {
                Command::new("cast")
                    .args([
                        "rpc",
                        "anvil_mine",
                        &blocks.to_string(),
                        "--rpc-url",
                        rpc_url,
                    ])
                    .stdout(Stdio::null())
                    .output()
                    .expect("Failed to execute command");
            }
            mine_anvil_block(&config.http_rpc_url(), current_block_number);
        }

        let avs_launcher = DefaultAvsLauncher::new();
        let avs_builder = AvsBuilder::new(config);
        let _ = avs_launcher.launch_avs(avs_builder).await;

        Ok(())
    }
}

/// Registers operator on eigenlayer and avs
#[allow(clippy::too_many_arguments)]
pub async fn register_operator_with_el_and_deposit_tokens_in_strategy(
    metadata_uri: String,
    allocation_delay: u32,
    operator_pvt_key: Option<String>,
    rpc_url: String,
    ecdsa_keystore_path: String,
    ecdsa_keystore_password: String,
    permission_controller_address: Address,
    rewards_coordinator: Address,
    allocation_manager: Address,
    registry_coordinator_address: Address,
    delegation_manager_address: Address,
    avs_directory_address: Address,
    strategy_manager_address: Address,
    erc20_strategy_address: Address,
    deposit_tokens: U256,
) -> eyre::Result<()> {
    let signer;
    if let Some(operator_key) = operator_pvt_key {
        signer = PrivateKeySigner::from_str(&operator_key)?;
    } else {
        signer = LocalSigner::decrypt_keystore(ecdsa_keystore_path, ecdsa_keystore_password)?;
    }
    let s = signer.to_field_bytes();
    let el_chain_reader = ELChainReader::new(
        get_logger(),
        Some(allocation_manager),
        delegation_manager_address,
        rewards_coordinator,
        avs_directory_address,
        Some(permission_controller_address),
        rpc_url.clone(),
    );
    let el_chain_writer = ELChainWriter::new(
        strategy_manager_address,
        rewards_coordinator,
        Some(permission_controller_address),
        Some(allocation_manager),
        registry_coordinator_address,
        el_chain_reader.clone(),
        rpc_url.clone(),
        hex::encode(s).to_string(),
    );
    let operator_details = Operator {
        address: signer.address(),
        delegation_approver_address: signer.address(),
        staker_opt_out_window_blocks: Some(0),
        metadata_url: metadata_uri,
        allocation_delay: Some(allocation_delay),
        _deprecated_earnings_receiver_address: None,
    };
    let is_already_registered = el_chain_reader
        .is_operator_registered(signer.address())
        .await?;
    if !is_already_registered {
        let _ = el_chain_writer
            .register_as_operator(operator_details)
            .await?; // register operator in delegation manager on EigenLayer
        deposit_into_strategy(erc20_strategy_address, deposit_tokens, el_chain_writer).await?;
    }
    Ok(())
}

/// set allocation delay
pub async fn set_allocation_delay(
    allocation_delay: u32,
    allocation_manager: Address,
    operator_pvt_key: Option<String>,
    ecdsa_keystore_path: String,
    ecdsa_keystore_password: String,
    rpc_url: &str,
) -> eyre::Result<FixedBytes<32>> {
    let signer;
    if let Some(operator_key) = operator_pvt_key {
        signer = PrivateKeySigner::from_str(&operator_key)?;
    } else {
        signer = LocalSigner::decrypt_keystore(ecdsa_keystore_path, ecdsa_keystore_password)?;
    }
    let s = signer.to_field_bytes();
    let pvt_key = hex::encode(s).to_string();

    let allocation_manager =
        AllocationManager::new(allocation_manager, get_signer(&pvt_key, rpc_url));
    Ok(allocation_manager
        .setAllocationDelay(signer.address(), allocation_delay)
        .send()
        .await?
        .get_receipt()
        .await?
        .transaction_hash)
}

/// Creates Total Delegated stake
#[allow(clippy::too_many_arguments)]
pub async fn create_total_delegated_stake_quorum(
    strategy_address: Address,
    registry_coordinator_address: Address,
    operator_pvt_key: Option<String>,
    ecdsa_keystore_path: String,
    ecdsa_keystore_password: String,
    rpc_url: &str,
) -> eyre::Result<FixedBytes<32>> {
    let signer;
    if let Some(operator_key) = operator_pvt_key {
        signer = PrivateKeySigner::from_str(&operator_key)?;
    } else {
        signer = LocalSigner::decrypt_keystore(ecdsa_keystore_path, ecdsa_keystore_password)?;
    }
    let s = signer.to_field_bytes();
    let pvt_key = hex::encode(s).to_string();

    let registry_coordinator_instance =
        RegistryCoordinator::new(registry_coordinator_address, get_signer(&pvt_key, rpc_url));

    let operator_set_param = OperatorSetParam {
        maxOperatorCount: 3,
        kickBIPsOfOperatorStake: 100,
        kickBIPsOfTotalStake: 1000,
    };
    let minimum_stake: U96 = U96::from(0);
    let strategy_params = vec![StrategyParams {
        strategy: strategy_address,
        multiplier: U96::from(1),
    }];
    let s = registry_coordinator_instance
        .createTotalDelegatedStakeQuorum(operator_set_param, minimum_stake, strategy_params)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap()
        .transaction_hash;
    Ok(s)
}

/// modify allocation for the operator for the particular operator set id
#[allow(clippy::too_many_arguments)]
pub async fn modify_allocation_for_operator(
    operator_set_id: u32,
    allocation_manager: Address,
    operator_pvt_key: Option<String>,
    ecdsa_keystore_path: String,
    ecdsa_keystore_password: String,
    rpc_url: &str,
    avs: Address,
    strategies: Vec<Address>,
    new_magnitude: Vec<u64>,
) -> eyre::Result<FixedBytes<32>> {
    let signer;
    if let Some(operator_key) = operator_pvt_key {
        signer = PrivateKeySigner::from_str(&operator_key)?;
    } else {
        signer = LocalSigner::decrypt_keystore(ecdsa_keystore_path, ecdsa_keystore_password)?;
    }
    let s = signer.to_field_bytes();
    let pvt_key = hex::encode(s).to_string();
    let allocation_manager_instance =
        AllocationManager::new(allocation_manager, get_signer(&pvt_key, rpc_url));
    let allocate_params = vec![AllocateParams {
        operatorSet: OperatorSet {
            avs,
            id: operator_set_id,
        },
        strategies,
        newMagnitudes: new_magnitude,
    }];
    Ok(allocation_manager_instance
        .modifyAllocations(signer.address(), allocate_params)
        .send()
        .await?
        .get_receipt()
        .await?
        .transaction_hash)
}

/// registers the operator to the operator set by calling the allocation manager
#[allow(clippy::too_many_arguments)]
pub async fn register_for_operator_sets(
    operator_set_id: u32,
    bls_key_pair: BlsKeyPair,
    permission_controller_address: Address,
    registry_coordinator_address: Address,
    allocation_manager: Address,
    operator_pvt_key: Option<String>,
    ecdsa_keystore_path: String,
    ecdsa_keystore_password: String,
    rpc_url: &str,
    avs: Address,
    socket: String,
) -> eyre::Result<FixedBytes<32>> {
    let signer;
    if let Some(operator_key) = operator_pvt_key {
        signer = PrivateKeySigner::from_str(&operator_key)?;
    } else {
        signer = LocalSigner::decrypt_keystore(ecdsa_keystore_path, ecdsa_keystore_password)?;
    }
    let s = signer.to_field_bytes();
    let pvt_key = hex::encode(s).to_string();
    let el_chain_reader = ELChainReader::new(
        get_logger(),
        Some(allocation_manager),
        Address::ZERO,
        Address::ZERO,
        Address::ZERO,
        None,
        rpc_url.to_string(),
    ); // zero address means we don't require that address for registering to operator sets
    let el_chain_writer = ELChainWriter::new(
        Address::ZERO,
        Address::ZERO,
        Some(permission_controller_address),
        Some(allocation_manager),
        registry_coordinator_address,
        el_chain_reader,
        rpc_url.to_string(),
        pvt_key,
    );

    Ok(el_chain_writer
        .register_for_operator_sets(
            signer.address(),
            avs,
            [operator_set_id].to_vec(),
            bls_key_pair,
            &socket,
        )
        .await?)
}

/// Deposit into strategy
///
/// # Arguments
///
/// * `strategy_address` - The address of the strategy
/// * `amount` - The amount to deposit
/// * `el_reader` - The EL chain reader
/// * `el_writer` - The EL chain writer
pub async fn deposit_into_strategy(
    strategy_address: Address,
    amount: U256,
    el_writer: ELChainWriter,
) -> Result<(), ElContractsError> {
    el_writer
        .deposit_erc20_into_strategy(strategy_address, amount)
        .await?;
    Ok(())
}
