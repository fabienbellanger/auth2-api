//! List of user use cases
//!
pub mod create_user;
pub mod get_access_token;
pub mod get_users;

use crate::domain::entities::user::UserId;
use crate::domain::repositories::user::UserRepository;
use crate::domain::use_cases::user::get_access_token::GetAccessTokenUseCase;
use crate::domain::value_objects::datetime::{UtcDateTime, UtcDateTimeError};
use crate::domain::value_objects::email::{Email, EmailError};
use crate::domain::value_objects::id::IdError;
use crate::domain::value_objects::password::{Password, PasswordError};
use create_user::CreateUserUseCase;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct UserUseCases<U: UserRepository> {
    pub create_user: CreateUserUseCase<U>,
    pub get_access_token: GetAccessTokenUseCase<U>,
}

impl<U: UserRepository> UserUseCases<U> {
    /// Create a new user use cases
    pub fn new(user_repository: U) -> Self {
        Self {
            create_user: CreateUserUseCase::new(user_repository.clone()),
            get_access_token: GetAccessTokenUseCase::new(user_repository),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum UserUseCaseError {
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    #[error("{0}")]
    InvalidId(String),

    #[error("{0}")]
    InvalidEmail(String),

    #[error("{0}")]
    InvalidPassword(String),

    #[error("{0}")]
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
    pub password: Password,
    pub lastname: String,
    pub firstname: String,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
}
