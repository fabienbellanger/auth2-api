//! Error conversion for user handlers

use crate::domain::use_cases::user::create_user::UserCreationError;
use crate::domain::use_cases::user::get_access_token::GetAccessTokenError;
use crate::infrastructure::api::response::ApiError;

impl From<UserCreationError> for ApiError {
    fn from(value: UserCreationError) -> Self {
        match value {
            UserCreationError::InvalidEmail(msg) => ApiError::BadRequest(msg),
            UserCreationError::InvalidPassword(msg) => ApiError::BadRequest(msg),
            UserCreationError::InvalidId() => ApiError::InternalServerError("User creation error".to_string()),
            UserCreationError::DatabaseError() => ApiError::InternalServerError("User creation error".to_string()),
        }
    }
}

impl From<GetAccessTokenError> for ApiError {
    fn from(value: GetAccessTokenError) -> Self {
        match value {
            GetAccessTokenError::InvalidPassword() => ApiError::BadRequest("Invalid password".to_string()),
            GetAccessTokenError::IncorrectPassword() => ApiError::Unauthorized("User not found".to_string()),
            GetAccessTokenError::UserNotFound() => ApiError::Unauthorized("User not found".to_string()),
            GetAccessTokenError::InvalidId() => ApiError::InternalServerError("Get access token error".to_string()),
            GetAccessTokenError::DatabaseError() => ApiError::InternalServerError("Get access token error".to_string()),
            GetAccessTokenError::TokenGenerationError() => {
                ApiError::InternalServerError("Get access token error".to_string())
            }
        }
    }
}
