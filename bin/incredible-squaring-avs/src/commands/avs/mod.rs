use clap::{value_parser, Args, Parser};
use std::ffi::OsString;
use std::fmt;

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
    #[arg(long, value_name = "RPC_URL", global = true)]
    rpc_url: Option<String>,

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
        Ok(())
    }
}
