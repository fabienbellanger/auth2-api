//! Error conversion for scope handlers

use crate::domain::use_cases::scope::ScopeUseCaseError;
use crate::infrastructure::api::response::ApiError;

impl From<ScopeUseCaseError> for ApiError {
    fn from(value: ScopeUseCaseError) -> Self {
        match value {
            ScopeUseCaseError::InvalidId(err) => ApiError::InternalServerError(err),
            ScopeUseCaseError::InvalidApplicationId(err) => ApiError::InternalServerError(err),
            ScopeUseCaseError::DatabaseError(msg) => ApiError::InternalServerError(msg),
            ScopeUseCaseError::FromModelError() => ApiError::InternalServerError("Internal server error".to_string()),
            ScopeUseCaseError::ScopeNotFound() => ApiError::NotFound("Application not found".to_string()),
        }
    }
}
