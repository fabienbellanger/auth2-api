use auth2_api::infrastructure::api::server::start_server;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    start_server().await.unwrap();

    Ok(())
}
