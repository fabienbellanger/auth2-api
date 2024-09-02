//! User handler DTO

use crate::domain::use_cases::user::create_user::{
    CreateUserUseCaseRequest, CreateUserUseCaseResponse, UserCreationError,
};
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;
use crate::infrastructure::api::response::ApiError;
use serde::{Deserialize, Serialize};
use validator::Validate;

impl From<UserCreationError> for ApiError {
    fn from(value: UserCreationError) -> Self {
        match value {
            UserCreationError::InvalidId(err) => ApiError::InternalServerError(err.to_string()),
            UserCreationError::InvalidEmail(err) => ApiError::BadRequest(err.to_string()),
            UserCreationError::InvalidPassword(err) => ApiError::BadRequest(err.to_string()),
            UserCreationError::DatabaseError(err) => ApiError::InternalServerError(err.to_string()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub lastname: String,
    pub firstname: String,
}

impl TryFrom<CreateUserRequest> for CreateUserUseCaseRequest {
    type Error = UserCreationError;

    fn try_from(value: CreateUserRequest) -> Result<Self, Self::Error> {
        let email = Email::new(&value.email).map_err(|err| UserCreationError::InvalidEmail(err.to_string()))?;
        let password =
            Password::new(&value.password, false).map_err(|err| UserCreationError::InvalidPassword(err.to_string()))?;

        Ok(Self {
            email,
            password,
            lastname: value.lastname,
            firstname: value.firstname,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateUserResponse {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<CreateUserUseCaseResponse> for CreateUserResponse {
    fn from(value: CreateUserUseCaseResponse) -> Self {
        Self {
            id: value.id.to_string(),
            lastname: value.lastname,
            firstname: value.firstname,
            email: value.email.value(),
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
        }
    }
}
