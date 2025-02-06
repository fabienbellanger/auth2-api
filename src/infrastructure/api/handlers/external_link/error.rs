//! Error conversion for external link handlers

use crate::{domain::use_cases::external_link::ExternalLinkUseCaseError, infrastructure::api::response::ApiError};

impl From<ExternalLinkUseCaseError> for ApiError {
    fn from(value: ExternalLinkUseCaseError) -> Self {
        match value {
            ExternalLinkUseCaseError::DatabaseError(msg) => ApiError::InternalServerError(msg),
            ExternalLinkUseCaseError::InvalidId() => ApiError::InternalServerError(value.to_string()),
            ExternalLinkUseCaseError::ExternalLinkNotFound() => ApiError::NotFound(value.to_string()),
            ExternalLinkUseCaseError::FromModelError() => {
                ApiError::InternalServerError("Internal server error".to_string())
            }
        }
    }
}
