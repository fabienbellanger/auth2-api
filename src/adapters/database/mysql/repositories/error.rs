//! MySQL repositories errors

use crate::domain::use_cases::application::ApplicationUseCaseError;
use crate::domain::use_cases::external_link::ExternalLinkUseCaseError;
use crate::domain::use_cases::scope::ScopeUseCaseError;
use crate::domain::use_cases::user::UserUseCaseError;

impl From<sqlx::error::Error> for UserUseCaseError {
    fn from(err: sqlx::error::Error) -> Self {
        error!(error = %err, "Database error");
        Self::DatabaseError("Database error".to_string())
    }
}

impl From<sqlx::error::Error> for ApplicationUseCaseError {
    fn from(err: sqlx::error::Error) -> Self {
        error!(error = %err, "Database error");
        Self::DatabaseError("Database error".to_string())
    }
}

impl From<sqlx::error::Error> for ScopeUseCaseError {
    fn from(err: sqlx::error::Error) -> Self {
        error!(error = %err, "Database error");
        Self::DatabaseError("Database error".to_string())
    }
}

impl From<sqlx::error::Error> for ExternalLinkUseCaseError {
    fn from(err: sqlx::error::Error) -> Self {
        error!(error = %err, "Database error");
        Self::DatabaseError("Database error".to_string())
    }
}
