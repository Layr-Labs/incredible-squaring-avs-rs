use alloy::primitives::Address;
use clap::{value_parser, Args, Parser};
use incredible_config::IncredibleConfig;
use incredible_operator::builder::OperatorBuilder;
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
    registry_coordinator_address: Address,

    /// Aggregator Ip address
    #[arg(long, value_name = "AGGREGATOR_IP_ADDRESS")]
    aggregator_ip_address: String,

    /// bls keystore path
    #[arg(long, value_name = "BLS_KEYSTORE_PATH")]
    bls_keystore_path: String,

    /// bls keystore password
    #[arg(long, value_name = "BLS_KEYSTORE_PASSWORD")]
    bls_keystore_password: String,

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
        debug!("Executing AVS command");
        debug!("chain id : {:?}", self.chain_id);
        debug!("rpc url : {:?}", self.rpc_url);
        debug!("ecdsa key store path {:?}", self.ecdsa_keystore_path);
        debug!("ecdsa key password:{:?}", self.ecdsa_keystore_password);
        debug!("bls keystore path : {:?}", self.bls_keystore_path);
        debug!("bls keystore password : {:?}", self.bls_keystore_password);
        let mut config = IncredibleConfig::default();
        config.set_chain_id(self.chain_id);
        config.set_rpc_url(self.rpc_url);
        config.set_ecdsa_keystore_path(self.ecdsa_keystore_path);
        config.set_ecdsa_keystore_pasword(self.ecdsa_keystore_password);
        config.set_aggregator_ip_address(self.aggregator_ip_address);
        config.set_bls_keystore_path(self.bls_keystore_path);
        config.set_bls_keystore_password(self.bls_keystore_password);
        let _ = OperatorBuilder::build(config);
        Ok(())
    }
}
