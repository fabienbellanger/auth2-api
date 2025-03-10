use auth2_api::config::Config;
use auth2_api::infrastructure::cli;
use auth2_api::infrastructure::cli::error::CliError;

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let worker_threads = Config::from_env()?.worker_threads;

    let mut rt = tokio::runtime::Builder::new_multi_thread();
    if worker_threads > 0 {
        rt.worker_threads(worker_threads);
    }
    rt.enable_all()
        .build()
        .map_err(|err| CliError::RuntimeError(err.to_string()))?
        .block_on(async { cli::start().await })
}
