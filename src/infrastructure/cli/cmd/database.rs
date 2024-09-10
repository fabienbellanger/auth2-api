//! Database commands

use crate::adapters::database::mysql::repositories::refresh_token::RefreshTokenMysqlRepository;
use crate::adapters::database::mysql::Db;
use crate::adapters::database::GenericDb;
use crate::config::Config;
use crate::domain::use_cases::database::clean_expired_refresh_tokens::{
    CleanExpiredRefreshTokens, CleanExpiredRefreshTokensUseCaseRequest,
};
use crate::infrastructure::cli::error::CliError;

/// Clean expired data
pub async fn clean_data() -> Result<(), CliError> {
    println!("\nCleaning database...");

    // Load configuration
    let config = Config::from_env().map_err(|err| CliError::ConfigError(err.to_string()))?;
    println!("\n► Configuration.....OK");

    // Database
    let db = Db::new(&config)
        .await
        .map_err(|err| CliError::DatabaseError(err.to_string()))?;
    println!("► Database..........OK");

    // Refresh token use case
    let refresh_token_repository = RefreshTokenMysqlRepository::new(db.clone());
    let refresh_token_use_case = CleanExpiredRefreshTokens::new(refresh_token_repository);
    let affected_rows = refresh_token_use_case
        .call(CleanExpiredRefreshTokensUseCaseRequest())
        .await
        .map_err(|err| CliError::DatabaseError(err.to_string()))?;
    println!("\n→ Expired refresh tokens deleted: {}", affected_rows.deleted);

    Ok(())
}
