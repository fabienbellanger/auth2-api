//! CLI module

mod cmd;
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

    /// Register user
    #[clap(about = "Create a new user", long_about = None)]
    Register {
        /// User lastname
        #[clap(
            required = true,
            short = 'l',
            long,
            value_name = "Lastname",
            num_args = 1,
            help = "Lastname"
        )]
        lastname: String,

        /// User firstname
        #[clap(
            required = true,
            short = 'f',
            long,
            value_name = "Firstname",
            num_args = 1,
            help = "Firstname"
        )]
        firstname: String,

        /// User email
        #[clap(
            required = true,
            short = 'e',
            long,
            value_name = "Email",
            num_args = 1,
            help = "Email"
        )]
        email: String,

        /// User password (at least 8 characters)
        #[clap(
            required = true,
            short = 'p',
            long,
            value_name = "Password",
            num_args = 1,
            help = "Password (at least 8 characters)"
        )]
        password: String,
        // /// User scopes (separated by commas)
        // #[clap(
        //     required = false,
        //     short = 's',
        //     long,
        //     value_delimiter = ',',
        //     value_name = "Scopes",
        //     help = "Scopes separated by commas"
        // )]
        // scopes: Option<Vec<String>>,
    },

    /// Clean expired database data
    #[clap(about = "Clean expired database data", long_about = None)]
    CleanDatabase,
}

/// Start CLI
pub async fn start() -> Result<(), CliError> {
    let args = Cli::parse();
    match &args.commands {
        Commands::Serve => start_server().await.map_err(CliError::from),
        Commands::Register {
            lastname,
            firstname,
            email,
            password,
        } => cmd::user::register(lastname, firstname, email, password).await,
        Commands::CleanDatabase => cmd::database::clean_data().await,
    }
}
