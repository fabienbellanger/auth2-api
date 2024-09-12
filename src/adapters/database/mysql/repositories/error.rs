//! MySQL repositories errors

use crate::domain::use_cases::user::UserUseCaseError;

impl From<sqlx::error::Error> for UserUseCaseError {
    fn from(err: sqlx::error::Error) -> Self {
        error!(error = %err, "Database error");
        UserUseCaseError::DatabaseError("Database error".to_string())
    }
}
