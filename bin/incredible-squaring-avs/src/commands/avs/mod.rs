use clap::{value_parser, Args, Parser};
use incredible_config::IncredibleConfig;
use std::ffi::OsString;
use std::fmt;
use incredible_operator::builder::OperatorBuilder;

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
    #[arg(long, value_name = "RPC_URL")]
    rpc_url: String,

    /// ECDSA key store path file
    #[arg(long, value_name = "ECDSA_KEYSTORE_PATH")]
    ecdsa_keystore_path: String,

    /// ECDSA keystore path  password 
    #[arg(long, value_name = "ECDSA_KEYSTORE_PASSWORD")]
    ecdsa_keystore_password: String,

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
    pub async fn execute(self: Box<Self>) -> eyre::Result<()> {
        println!("Executing AVS command");
        println!("chain id : {:?}", self.chain_id);
        println!("rpc url : {:?}", self.rpc_url);
        println!("ecdsa key store path {:?}",self.ecdsa_keystore_path);
        println!("ecdsa key password:{:?}",self.ecdsa_keystore_password);
        let mut config = IncredibleConfig::default();
        config.set_chain_id(self.chain_id);
        config.set_rpc_url(self.rpc_url);
        config.set_ecdsa_keystore_path(self.ecdsa_keystore_path);
        config.set_ecdsa_keystore_pasword(self.ecdsa_keystore_password);
        let operator = OperatorBuilder::default();
        let _ = operator.build(config);
        Ok(())
    }
}
