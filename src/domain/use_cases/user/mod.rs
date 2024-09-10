//! List of user use cases

pub mod clean_expired_refresh_tokens;
pub mod create_user;
pub mod delete_user;
pub mod get_access_token;
pub mod get_user;
pub mod get_users;
pub mod refresh_token;

use crate::domain::entities::refresh_token::RefreshTokenError;
use crate::domain::entities::user::UserId;
use crate::domain::repositories::refresh_token::RefreshTokenRepository;
use crate::domain::repositories::user::UserRepository;
use crate::domain::use_cases::user::delete_user::DeleteUserUseCase;
use crate::domain::use_cases::user::get_access_token::GetAccessTokenUseCase;
use crate::domain::use_cases::user::get_user::GetUserUseCase;
use crate::domain::use_cases::user::get_users::GetUsersUseCase;
use crate::domain::use_cases::user::refresh_token::RefreshTokenUseCase;
use crate::domain::value_objects::datetime::{UtcDateTime, UtcDateTimeError};
use crate::domain::value_objects::email::{Email, EmailError};
use crate::domain::value_objects::id::IdError;
use crate::domain::value_objects::password::PasswordError;
use create_user::CreateUserUseCase;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct UserUseCases<U: UserRepository, T: RefreshTokenRepository> {
    pub create_user: CreateUserUseCase<U>,
    pub get_access_token: GetAccessTokenUseCase<U, T>,
    pub get_users: GetUsersUseCase<U>,
    pub get_user: GetUserUseCase<U>,
    pub delete_user: DeleteUserUseCase<U>,
    pub refresh_token: RefreshTokenUseCase<T>,
}

impl<U: UserRepository, T: RefreshTokenRepository> UserUseCases<U, T> {
    /// Create a new user use cases
    pub fn new(user_repository: U, refresh_token_repository: T) -> Self {
        Self {
            create_user: CreateUserUseCase::new(user_repository.clone()),
            get_access_token: GetAccessTokenUseCase::new(user_repository.clone(), refresh_token_repository.clone()),
            get_users: GetUsersUseCase::new(user_repository.clone()),
            get_user: GetUserUseCase::new(user_repository.clone()),
            delete_user: DeleteUserUseCase::new(user_repository),
            refresh_token: RefreshTokenUseCase::new(refresh_token_repository),
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

    #[error("Access token generation error")]
    AccessTokenGenerationError(),

    #[error("Unauthorized")]
    Unauthorized(),

    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    #[error("Invalid user email: {0}")]
    InvalidEmail(String),

    #[error("Invalid user password: {0}")]
    InvalidPassword(String),

    #[error("Invalid UTC datetime: {0}")]
    InvalidUtcDateTime(String),

    #[error("Refresh token creation error: {0}")]
    RefreshTokenCreationError(String),

    #[error("Invalid Refresh token creation error")]
    InvalidRefreshToken(),

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

impl From<RefreshTokenError> for UserUseCaseError {
    fn from(err: RefreshTokenError) -> Self {
        UserUseCaseError::RefreshTokenCreationError(err.to_string())
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
