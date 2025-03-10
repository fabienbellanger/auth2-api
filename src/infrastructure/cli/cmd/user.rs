//! User CLI commands

use crate::adapters::database::GenericDb;
use crate::adapters::database::mysql::Db;
use crate::adapters::database::mysql::repositories::password_reset::PasswordResetMysqlRepository;
use crate::adapters::database::mysql::repositories::refresh_token::RefreshTokenMysqlRepository;
use crate::adapters::database::mysql::repositories::user::UserMysqlRepository;
use crate::adapters::email::EmailAdapter;
use crate::config::Config;
use crate::domain::entities::email::EmailConfig;
use crate::domain::use_cases::user::UserUseCases;
use crate::domain::use_cases::user::create_user::CreateUserUseCaseRequest;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;
use crate::infrastructure::cli::error::CliError;

/// Register a new user
pub async fn register(lastname: &str, firstname: &str, email: &str, password: &str) -> Result<(), CliError> {
    println!("\nCreating new user...");

    // Load configuration
    let config = Config::from_env().map_err(|err| CliError::ConfigError(err.to_string()))?;
    println!("\n► Configuration.....OK");

    // Database
    let db = Db::new(&config)
        .await
        .map_err(|err| CliError::DatabaseError(err.to_string()))?;
    println!("► Database..........OK");

    // Email service
    let email_service = EmailAdapter::new(EmailConfig::from(config.clone()));

    // User use case
    let user_repository = UserMysqlRepository::new(db.clone());
    let refresh_token_repository = RefreshTokenMysqlRepository::new(db.clone());
    let password_reset_repository = PasswordResetMysqlRepository::new(db.clone());
    let user_use_case = UserUseCases::new(
        user_repository,
        refresh_token_repository,
        password_reset_repository,
        email_service,
    );

    let email = Email::new(email).map_err(|err| CliError::InvalidArguments(err.to_string()))?;
    let password = Password::new(password, false).map_err(|err| CliError::InvalidArguments(err.to_string()))?;

    let response = user_use_case
        .create_user
        .call(CreateUserUseCaseRequest {
            lastname: lastname.trim().to_string(),
            firstname: firstname.trim().to_string(),
            email,
            password,
        })
        .await
        .map_err(|err| CliError::DatabaseError(err.to_string()))?;
    println!("\n→ User creation success with ID: {}", response.id);

    Ok(())
}
