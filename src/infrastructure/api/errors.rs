//! Errors that can be returned by the API.

use crate::adapters::database::DatabaseError;
use crate::config::ConfigError;
use crate::domain::services::security::jwt::JwtError;
use crate::domain::value_objects::email::EmailError;
use crate::domain::value_objects::id::IdError;
use crate::domain::value_objects::password::PasswordError;
use crate::infrastructure::api::response::ApiError;
use axum::BoxError;
use tower::timeout::error::Elapsed;

/// Timeout error
pub async fn timeout_error(err: BoxError) -> Result<(), ApiError> {
    if err.is::<Elapsed>() {
        Err(ApiError::Timeout)
    } else {
        Err(ApiError::InternalServerError(err.to_string()))
    }
}

impl From<JwtError> for ApiError {
    fn from(value: JwtError) -> Self {
        Self::InternalServerError(value.to_string())
    }
}

impl From<ConfigError> for ApiError {
    fn from(value: ConfigError) -> Self {
        Self::InternalServerError(value.to_string())
    }
}

impl From<DatabaseError> for ApiError {
    fn from(value: DatabaseError) -> Self {
        Self::InternalServerError(value.to_string())
    }
}

impl From<IdError> for ApiError {
    fn from(value: IdError) -> Self {
        Self::BadRequest(value.to_string())
    }
}

impl From<EmailError> for ApiError {
    fn from(value: EmailError) -> Self {
        Self::BadRequest(value.to_string())
    }
}

impl From<PasswordError> for ApiError {
    fn from(value: PasswordError) -> Self {
        Self::BadRequest(value.to_string())
    }
}
