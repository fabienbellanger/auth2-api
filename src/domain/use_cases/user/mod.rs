//! List of user use cases

pub mod create_user;
pub mod get_access_token;
pub mod get_user;
pub mod get_users;

use crate::domain::entities::user::UserId;
use crate::domain::repositories::user::UserRepository;
use crate::domain::use_cases::user::get_access_token::GetAccessTokenUseCase;
use crate::domain::use_cases::user::get_user::GetUserUseCase;
use crate::domain::use_cases::user::get_users::GetUsersUseCase;
use crate::domain::value_objects::datetime::{UtcDateTime, UtcDateTimeError};
use crate::domain::value_objects::email::{Email, EmailError};
use crate::domain::value_objects::id::IdError;
use crate::domain::value_objects::password::PasswordError;
use create_user::CreateUserUseCase;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct UserUseCases<U: UserRepository> {
    pub create_user: CreateUserUseCase<U>,
    pub get_access_token: GetAccessTokenUseCase<U>,
    pub get_users: GetUsersUseCase<U>,
    pub get_user: GetUserUseCase<U>,
}

impl<U: UserRepository> UserUseCases<U> {
    /// Create a new user use cases
    pub fn new(user_repository: U) -> Self {
        Self {
            create_user: CreateUserUseCase::new(user_repository.clone()),
            get_access_token: GetAccessTokenUseCase::new(user_repository.clone()),
            get_users: GetUsersUseCase::new(user_repository.clone()),
            get_user: GetUserUseCase::new(user_repository),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum UserUseCaseError {
    #[error("Invalid user id")]
    InvalidId(),

    #[error("Incorrect user password")]
    IncorrectPassword(),

    #[error("User not found")]
    UserNotFound(),

    #[error("Token generation error")]
    TokenGenerationError(),

    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    #[error("Invalid user email: {0}")]
    InvalidEmail(String),

    #[error("Invalid user password: {0}")]
    InvalidPassword(String),

    #[error("Invalid UTC datetime: {0}")]
    InvalidUtcDateTime(String),

    #[error("{0}")]
    DatabaseError(String),
}

impl From<IdError> for UserUseCaseError {
    fn from(err: IdError) -> Self {
        UserUseCaseError::InvalidEmail(err.to_string())
    }
}

impl From<EmailError> for UserUseCaseError {
    fn from(err: EmailError) -> Self {
        UserUseCaseError::InvalidEmail(err.to_string())
    }
}

impl From<PasswordError> for UserUseCaseError {
    fn from(err: PasswordError) -> Self {
        UserUseCaseError::InvalidPassword(err.to_string())
    }
}

impl From<UtcDateTimeError> for UserUseCaseError {
    fn from(err: UtcDateTimeError) -> Self {
        UserUseCaseError::InvalidUtcDateTime(err.to_string())
    }
}

/// User use case generic response
#[derive(Debug, Clone)]
pub struct UserUseCaseResponse {
    pub id: UserId,
    pub email: Email,
    pub lastname: String,
    pub firstname: String,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
}
