//! Error conversion for application handlers

use crate::domain::use_cases::application::ApplicationUseCaseError;
use crate::infrastructure::api::response::ApiError;

impl From<ApplicationUseCaseError> for ApiError {
    fn from(value: ApplicationUseCaseError) -> Self {
        match value {
            ApplicationUseCaseError::DatabaseError(msg) => ApiError::InternalServerError(msg),
            ApplicationUseCaseError::InvalidName(msg) => ApiError::BadRequest(msg),
            ApplicationUseCaseError::InvalidId() => ApiError::InternalServerError(value.to_string()),
            ApplicationUseCaseError::ApplicationNotFound() => ApiError::NotFound(value.to_string()),
            ApplicationUseCaseError::FromModelError() => {
                ApiError::InternalServerError("Internal server error".to_string())
            }
        }
    }
}
