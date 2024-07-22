use clap::Args;

/// log config
#[derive(Debug, Args)]
#[command(next_help_heading = "logging ")]
pub struct Logs {
    /// directory
    log_directory: String,
}
