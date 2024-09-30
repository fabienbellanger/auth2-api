//! Error conversion for scope handlers

use crate::domain::use_cases::scope::ScopeUseCaseError;
use crate::infrastructure::api::response::ApiError;

impl From<ScopeUseCaseError> for ApiError {
    fn from(value: ScopeUseCaseError) -> Self {
        match value {
            ScopeUseCaseError::InvalidId(msg) => ApiError::InternalServerError(msg),
            ScopeUseCaseError::InvalidApplicationId(msg) => ApiError::InternalServerError(msg),
            ScopeUseCaseError::DatabaseError(msg) => ApiError::InternalServerError(msg),
            ScopeUseCaseError::ScopeNotFound() => ApiError::NotFound(value.to_string()),
            ScopeUseCaseError::IdAlreadyExists() => ApiError::InternalServerError(value.to_string()),
            ScopeUseCaseError::FromModelError() => ApiError::InternalServerError("Internal server error".to_string()),
        }
    }
}
