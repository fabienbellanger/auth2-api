//! Error conversion for application handlers

use crate::domain::use_cases::application::ApplicationUseCaseError;
use crate::infrastructure::api::response::ApiError;

impl From<ApplicationUseCaseError> for ApiError {
    fn from(value: ApplicationUseCaseError) -> Self {
        match value {
            ApplicationUseCaseError::InvalidId() => {
                ApiError::InternalServerError("Application creation error".to_string())
            }
            ApplicationUseCaseError::DatabaseError(msg) => ApiError::InternalServerError(msg),
            ApplicationUseCaseError::InvalidName(msg) => ApiError::BadRequest(msg),
            ApplicationUseCaseError::FromModelError() => {
                ApiError::InternalServerError("Internal server error".to_string())
            }
            ApplicationUseCaseError::ApplicationNotFound() => ApiError::NotFound("Application not found".to_string()),
        }
    }
}
