//! MySQL repositories errors

use crate::domain::use_cases::application::ApplicationUseCaseError;
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
