//! CLI module

pub mod error;

use crate::infrastructure::api::server::start_server;
use crate::infrastructure::cli::error::CliError;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = crate::APP_NAME,
    version = clap::crate_version!(),
    author = clap::crate_authors!(),
)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start server
    #[clap(about = "Start Web server", long_about = None)]
    Serve,
}

/// Start CLI
pub async fn start() -> Result<(), CliError> {
    let args = Cli::parse();
    match &args.commands {
        Commands::Serve => start_server().await.map_err(|err| err.into()),
    }
}
