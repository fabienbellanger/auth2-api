//! Error conversion for user handlers

use crate::domain::use_cases::user::UserUseCaseError;
use crate::infrastructure::api::response::ApiError;

impl From<UserUseCaseError> for ApiError {
    fn from(value: UserUseCaseError) -> Self {
        match value {
            UserUseCaseError::InvalidEmail(msg) => ApiError::BadRequest(msg),
            UserUseCaseError::InvalidPassword(msg) => ApiError::BadRequest(msg),
            UserUseCaseError::InvalidUtcDateTime(msg) => ApiError::InternalServerError(msg),
            UserUseCaseError::InvalidArguments(msg) => ApiError::BadRequest(msg),
            UserUseCaseError::SendEmailError(msg) => ApiError::InternalServerError(msg),
            UserUseCaseError::DatabaseError(msg) => ApiError::InternalServerError(msg),
            UserUseCaseError::RefreshTokenCreationError(msg) => ApiError::InternalServerError(msg),
            UserUseCaseError::InvalidId() => ApiError::InternalServerError(value.to_string()),
            UserUseCaseError::UserNotFound() => ApiError::NotFound(value.to_string()),
            UserUseCaseError::Unauthorized() => ApiError::Unauthorized(value.to_string()),
            UserUseCaseError::ForgottenPasswordNotFound() => ApiError::NotFound(value.to_string()),
            UserUseCaseError::AccessTokenGenerationError() => ApiError::InternalServerError(value.to_string()),
            UserUseCaseError::InvalidRefreshToken() => ApiError::Unauthorized("Invalid refresh token".to_string()),
            UserUseCaseError::FromModelError() => ApiError::InternalServerError("Internal server error".to_string()),
            UserUseCaseError::IncorrectPassword() => ApiError::Unauthorized("User not found".to_string()),
        }
    }
}
