use auth2_api::infrastructure::cli;
use auth2_api::infrastructure::cli::error::CliError;

#[tokio::main]
async fn main() -> Result<(), CliError> {
    cli::start().await
}
