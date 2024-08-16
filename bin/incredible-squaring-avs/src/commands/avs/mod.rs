use alloy::primitives::Address;
use clap::{value_parser, Args, Parser};
use eigen_testing_utils::anvil_constants;
use incredible_avs::builder::{AvsBuilder, DefaultAvsLauncher, LaunchAvs};
use incredible_config::IncredibleConfig;
use std::ffi::OsString;
use std::fmt;
use tracing::debug;

/// No Additional arguments
#[derive(Debug, Clone, Copy, Default, Args)]
#[non_exhaustive]
pub struct NoArgs;

/// Starts incredible squaring
#[derive(Debug, Parser)]
pub struct AvsCommand<Ext: Args + fmt::Debug = NoArgs> {
    /// The EVM chain ID.
    #[arg(
    long,
    value_name = "CHAIN_ID",
    global = true,
    default_value_t = 1,
    value_parser = value_parser!(u16).range(1..)
)]
    chain_id: u16,

    /// The RPC URL of the node.
    #[arg(long, value_name = "RPC_URL",default_value = "http://localhost:8545", value_parser = clap::value_parser!(String))]
    rpc_url: String,

    /// ECDSA key store path file
    #[arg(long, value_name = "ECDSA_KEYSTORE_PATH")]
    ecdsa_keystore_path: String,

    /// ECDSA keystore path  password
    #[arg(long, value_name = "ECDSA_KEYSTORE_PASSWORD")]
    ecdsa_keystore_password: String,

    /// Registry coordinator address
    #[arg(long, value_name = "REGISTRY_COORDINATOR_ADDR")]
    registry_coordinator_address: Option<String>,

    /// Aggregator Ip address
    #[arg(long, value_name = "AGGREGATOR_IP_ADDRESS")]
    aggregator_ip_address: String,

    /// bls keystore path
    #[arg(long, value_name = "BLS_KEYSTORE_PATH")]
    bls_keystore_path: String,

    /// bls keystore password
    #[arg(long, value_name = "BLS_KEYSTORE_PASSWORD")]
    bls_keystore_password: String,

    /// No BLS flag to skip BLS keystore loading
    #[arg(
        long,
        value_name = "NO_BLS",
        help = "Skip BLS keystore loading",
        default_value_t = 1
    )]
    no_bls: u64,

    /// Operator Id
    #[arg(long, value_name = "OPERATOR_ID")]
    operator_id: String,

    /// Operator State retreiver
    #[arg(long, value_name = "OPERATOR_STATE_RETRIEVER_ADDRESS")]
    operator_state_retriever_addr: Option<String>,

    /// Operator Address
    #[arg(long, value_name = "OPERATOR_ADDRESS")]
    operator_address: String,

    /// additional arguments
    #[command(flatten, next_help_heading = "Extension")]
    pub ext: Ext,
}

/// Default Anvil configuration
#[derive(Debug)]
pub struct AnvilValues {
    registry_coordinator_address: Address,
    operator_state_retriever_address: Address,
}

impl AnvilValues {
    /// new
    pub fn new(registry_coordinator: Address, operator_state_retriever_address: Address) -> Self {
        Self {
            registry_coordinator_address: registry_coordinator,
            operator_state_retriever_address,
        }
    }
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
        let registry_coordinator_address_anvil =
            anvil_constants::get_registry_coordinator_address().await;
        let operator_state_retriever_address_anvil =
            anvil_constants::get_operator_state_retriever_address().await;
        let default_anvil = AnvilValues::new(
            registry_coordinator_address_anvil,
            operator_state_retriever_address_anvil,
        );

        debug!("Executing AVS command");
        debug!("chain id : {:?}", self.chain_id);
        debug!("rpc url : {:?}", self.rpc_url);
        debug!("ecdsa key store path {:?}", self.ecdsa_keystore_path);
        debug!("ecdsa key password:{:?}", self.ecdsa_keystore_password);
        debug!("bls keystore path : {:?}", self.bls_keystore_path);
        debug!("bls keystore password : {:?}", self.bls_keystore_password);
        let mut config = IncredibleConfig::default();

        let Self {
            chain_id,
            rpc_url,
            ecdsa_keystore_path,
            ecdsa_keystore_password,
            registry_coordinator_address,
            aggregator_ip_address,
            bls_keystore_path,
            bls_keystore_password,
            no_bls,
            operator_id,
            operator_state_retriever_addr,
            operator_address,
            ext,
        } = *self;
        config.set_chain_id(chain_id);
        config.set_http_rpc_url(rpc_url);
        config.set_ecdsa_keystore_path(ecdsa_keystore_path);
        config.set_ecdsa_keystore_pasword(ecdsa_keystore_password);
        config.set_aggregator_ip_address(aggregator_ip_address);
        config.set_bls_keystore_path(bls_keystore_path);
        config.set_bls_keystore_password(bls_keystore_password);
        // use value from config , if None , then use anvil
        config.set_registry_coordinator_addr(
            registry_coordinator_address
                .unwrap_or(default_anvil.registry_coordinator_address.to_string()),
        );
        config.set_operator_state_retriever(
            operator_state_retriever_addr
                .unwrap_or(default_anvil.operator_state_retriever_address.to_string()),
        );
        config.set_operator_id(operator_id);
        config.set_operator_address(operator_address);

        let avs_launcher = DefaultAvsLauncher::new();
        let avs_builder = AvsBuilder::new(config);
        let s = avs_launcher.launch_avs(avs_builder).await;

        println!("launch avs result: {:?}", s);

        Ok(())
    }
}
