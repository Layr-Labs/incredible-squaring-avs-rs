use crate::commands::avs::{AvsCommand, NoArgs};
use clap::{self, Args, Parser, Subcommand};
use incredible_cli_runner::IncredibleRunner;
use std::ffi::OsString;
use std::fmt;

/// Incredible squaring main entry point interface
#[derive(Debug, Parser)]
#[command(author, about = "Incredible squaring avs", long_about = None)]
pub struct Cli<Ext: clap::Args + fmt::Debug = NoArgs> {
    #[command(subcommand)]
    command: Commands<Ext>,
}

impl Cli {
    /// Parsers only the default CLI arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Parsers only the default CLI arguments from the given iterator
    pub fn try_parse_args_from<I, T>(itr: I) -> Result<Self, clap::error::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::try_parse_from(itr)
    }
}

impl<Ext: clap::Args + fmt::Debug + Send + Sync + 'static> Cli<Ext> {
    /// Execute the configured CLI command.
    pub fn start(self) -> eyre::Result<()> {
        let runner = IncredibleRunner::default();

        match self.command {
            Commands::Avs(command) => {
                runner.run_blocking_until_ctrl_c(Box::new(command).execute())?
            }
        }

        Ok(())
    }
}

/// Commands to be executed
#[derive(Debug, Subcommand)]
pub enum Commands<Ext: Args + fmt::Debug = NoArgs> {
    /// Avs command
    #[command(name = "start")]
    Avs(AvsCommand<Ext>),
}
