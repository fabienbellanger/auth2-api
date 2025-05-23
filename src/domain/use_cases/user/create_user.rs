//! User creation use case

use crate::domain::repositories::user::UserRepository;
use crate::domain::repositories::user::dto::CreateUserDtoRequest;
use crate::domain::use_cases::user::{UserUseCaseError, UserUseCaseResponse};
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;

#[derive(Debug, Clone)]
pub struct CreateUserUseCaseRequest {
    pub email: Email,
    pub password: Password,
    pub lastname: String,
    pub firstname: String,
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
    pub async fn call(&self, request: CreateUserUseCaseRequest) -> Result<UserUseCaseResponse, UserUseCaseError> {
        // TODO: Validation?

        let user = self.user_repository.create_user(CreateUserDtoRequest(request)).await?;

        Ok(user.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::mock::user::{INVALID_EMAIL, UserRepositoryMock, VALID_EMAIL};
    use fake::Fake;
    use fake::faker::internet::fr_fr::Password;

    #[tokio::test]
    async fn test_create_user_use_case() {
        let user_repository = UserRepositoryMock {};
        let use_case = CreateUserUseCase::new(user_repository);
        let password: String = Password(16..25).fake();

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
        let password: String = Password(16..25).fake();

        let request = CreateUserUseCaseRequest {
            email: Email::new(&INVALID_EMAIL).unwrap(),
            password: Password::new(&password, false).unwrap(),
            lastname: "Doe".to_string(),
            firstname: "Jane".to_string(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_err());
        if let Err(e) = response {
            assert_eq!(e, UserUseCaseError::DatabaseError("User creation error".to_string()));
        }
    }
}
