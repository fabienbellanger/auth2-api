//! User creation use case

use crate::domain::entities::user::UserId;
use crate::domain::repositories::user::dto::CreateUserDtoRequest;
use crate::domain::repositories::user::UserRepository;
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;
use thiserror::Error;

/// User creation errors
#[derive(Debug, Clone, PartialEq, Error)]
pub enum UserCreationError {
    #[error("Invalid user email: {0}")]
    InvalidEmail(String),

    #[error("Invalid user password: {0}")]
    InvalidPassword(String),

    #[error("Invalid user ID")]
    InvalidId(),

    #[error("User creation error")]
    DatabaseError(),
}

#[derive(Debug, Clone)]
pub struct CreateUserUseCaseRequest {
    pub email: Email,
    pub password: Password,
    pub lastname: String,
    pub firstname: String,
}

#[derive(Debug, Clone)]
pub struct CreateUserUseCaseResponse {
    pub id: UserId,
    pub email: Email,
    pub password: Password,
    pub lastname: String,
    pub firstname: String,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
}

#[derive(Debug, Clone)]
pub struct CreateUserUseCase<U: UserRepository> {
    user_repository: U,
}

impl<U: UserRepository> CreateUserUseCase<U> {
    /// Create a new use case
    pub fn new(user_repository: U) -> Self {
        Self { user_repository }
    }

    /// Create a new user
    #[instrument(skip(self), name = "create_user_use_case")]
    pub async fn call(
        &self,
        request: CreateUserUseCaseRequest,
    ) -> Result<CreateUserUseCaseResponse, UserCreationError> {
        // TODO: Validation?

        let user = self.user_repository.create_user(CreateUserDtoRequest(request)).await?;

        Ok(user.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::mock::user::{UserRepositoryMock, INVALID_EMAIL, VALID_EMAIL};
    use fake::faker::internet::fr_fr::Password;
    use fake::Fake;

    #[tokio::test]
    async fn test_create_user_use_case() {
        let user_repository = UserRepositoryMock {};
        let use_case = CreateUserUseCase::new(user_repository);
        let password: String = Password(8..12).fake();

        let request = CreateUserUseCaseRequest {
            email: Email::new(&VALID_EMAIL).unwrap(),
            password: Password::new(&password, false).unwrap(),
            lastname: "Doe".to_string(),
            firstname: "John".to_string(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_create_user_use_case_invalid_email() {
        let user_repository = UserRepositoryMock {};
        let use_case = CreateUserUseCase::new(user_repository);
        let password: String = Password(8..12).fake();

        let request = CreateUserUseCaseRequest {
            email: Email::new(&INVALID_EMAIL).unwrap(),
            password: Password::new(&password, false).unwrap(),
            lastname: "Doe".to_string(),
            firstname: "Jane".to_string(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_err());
        if let Err(e) = response {
            assert_eq!(e, UserCreationError::DatabaseError());
        }
    }
}
