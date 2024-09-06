//! MySQL repositories errors

use crate::domain::use_cases::user::{UserUseCaseError, UserUseCaseResponse};
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::id::Id;
use sqlx::mysql::MySqlRow;
use sqlx::Row;
use std::str::FromStr;

impl From<sqlx::error::Error> for UserUseCaseError {
    fn from(err: sqlx::error::Error) -> Self {
        error!(error = %err, "Database error");
        UserUseCaseError::DatabaseError("Database error".to_string())
    }
}

impl TryFrom<MySqlRow> for UserUseCaseResponse {
    type Error = UserUseCaseError;

    fn try_from(row: MySqlRow) -> Result<Self, Self::Error> {
        Ok(UserUseCaseResponse {
            id: Id::from_str(row.try_get("id")?)?,
            email: Email::new(row.try_get("email")?)?,
            lastname: row.try_get("lastname")?,
            firstname: row.try_get("firstname")?,
            created_at: UtcDateTime::new(row.try_get("created_at")?),
            updated_at: UtcDateTime::new(row.try_get("updated_at")?),
        })
    }
}
