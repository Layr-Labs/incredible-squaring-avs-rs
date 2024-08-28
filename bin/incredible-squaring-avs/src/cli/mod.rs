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



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_parse_avs_command() {
        let args = vec![
            "incredible-squaring-avs",
            "start",
            "--chain-id",
            "31337",
            "--ecdsa-keystore-path",
            "./crates/testing-utils/src/ecdsakeystore.json",
            "--ecdsa-keystore-password",
            "test",
            "--bls-keystore-path",
            "./crates/testing-utils/src/blskeystore.json",
            "--bls-keystore-password",
            "testpassword",
            "--operator-id",
            "0xb345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3",
            "--operator-address",
            "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
            "--register-operator",
            "--operator-to-avs-registration-sig-salt",
            "b345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3",
            "--socket",
            "hello",
            "--quorum-number",
            "00"
        ];

        // Parse the arguments into the `Cli` struct
        let cli: Cli = Cli::try_parse_args_from(args).unwrap();

      
    }

}