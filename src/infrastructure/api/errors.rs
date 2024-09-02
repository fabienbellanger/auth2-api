//! Errors that can be returned by the API.

use crate::adapters::database::DatabaseError;
use crate::config::ConfigError;
use crate::domain::services::security::jwt::JwtError;
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
