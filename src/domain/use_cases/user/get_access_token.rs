//! Get Access Token Use Case

use crate::domain::entities::access_token::{AccessToken, AccessTokenValue};
use crate::domain::repositories::user::dto::GetAccessTokenInformationDtoRequest;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::security::jwt::Jwt;
use crate::domain::services::security::payload::PayloadData;
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;
use thiserror::Error;

/// Get access token errors
#[derive(Debug, Clone, PartialEq, Error)]
pub enum GetAccessTokenError {
    #[error("Invalid user ID")]
    InvalidId(),

    #[error("Invalid user password")]
    InvalidPassword(),

    #[error("Incorrect user password")]
    IncorrectPassword(),

    #[error("User not found")]
    UserNotFound(),

    #[error("Token generation error")]
    TokenGenerationError(),

    #[error("Database error")]
    DatabaseError(),
}

// TODO: Add application_id and client_id later
#[derive(Debug, Clone)]
pub struct GetAccessTokenUseCaseRequest {
    /// User email
    pub email: Email,

    /// User password
    pub password: Password,

    /// JWT instance
    pub jwt: Jwt,
}

// TODO: Add refresh_token information later
#[derive(Debug, Clone)]
pub struct GetAccessTokenUseCaseResponse {
    /// Access token value
    pub access_token: AccessTokenValue,

    /// Access token expiration time
    pub access_token_expired_at: UtcDateTime,
}

impl From<AccessToken> for GetAccessTokenUseCaseResponse {
    fn from(value: AccessToken) -> Self {
        Self {
            access_token: value.token,
            access_token_expired_at: value.expired_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetAccessTokenUseCase<U: UserRepository> {
    #[allow(dead_code)]
    user_repository: U,
}

impl<U: UserRepository> GetAccessTokenUseCase<U> {
    /// Create a new use case
    pub fn new(user_repository: U) -> Self {
        Self { user_repository }
    }

    /// Generate a new access token
    #[instrument(skip(self), name = "get_access_token_use_case")]
    pub async fn call(
        &self,
        request: GetAccessTokenUseCaseRequest,
    ) -> Result<GetAccessTokenUseCaseResponse, GetAccessTokenError> {
        // TODO: Validation?

        let original_password = request
            .password
            .original()
            .ok_or(GetAccessTokenError::InvalidPassword())?;

        // Get user by email
        let resp = self
            .user_repository
            .get_access_token_information(GetAccessTokenInformationDtoRequest(request.email))
            .await?;

        // Check user password
        let user_id = match resp {
            Some(user) => {
                if user.password.verify(&original_password).is_ok() {
                    user.id
                } else {
                    Err(GetAccessTokenError::IncorrectPassword())?
                }
            }
            None => Err(GetAccessTokenError::UserNotFound())?,
        };

        // Generate access token
        let payload = PayloadData::new(user_id.to_string(), "".to_string(), "".to_string());
        let access_token = request.jwt.generate(payload).map_err(|err| {
            error!(error = %err, "Error generating access token");
            GetAccessTokenError::TokenGenerationError()
        })?;

        Ok(access_token.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::mock::user::{
        UserRepositoryMock, EMAIL_NOT_FOUND, INVALID_EMAIL, INVALID_PASSWORD, VALID_EMAIL, VALID_PASSWORD,
    };

    #[tokio::test]
    async fn test_get_access_token_use_case() {
        let user_repository = UserRepositoryMock {};
        let use_case = GetAccessTokenUseCase::new(user_repository);
        let password = Password::new(VALID_PASSWORD, false).unwrap();
        let email = Email::new(VALID_EMAIL).unwrap();
        let jwt = Jwt::init("HS256", 1, 1, Some("secret"), None, None).unwrap();

        let request = GetAccessTokenUseCaseRequest { email, password, jwt };

        let response = use_case.call(request).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_get_access_token_use_case_invalid_email() {
        let user_repository = UserRepositoryMock {};
        let use_case = GetAccessTokenUseCase::new(user_repository);
        let password = Password::new(VALID_PASSWORD, false).unwrap();
        let email = Email::new(INVALID_EMAIL).unwrap();
        let jwt = Jwt::init("HS256", 1, 1, Some("secret"), None, None).unwrap();

        let request = GetAccessTokenUseCaseRequest { email, password, jwt };

        let response = use_case.call(request).await;
        assert!(response.is_err());
        if let Err(e) = response {
            assert_eq!(e, GetAccessTokenError::DatabaseError());
        }
    }

    #[tokio::test]
    async fn test_get_access_token_use_case_incorrect_password() {
        let user_repository = UserRepositoryMock {};
        let use_case = GetAccessTokenUseCase::new(user_repository);
        let password = Password::new(INVALID_PASSWORD, false).unwrap();
        let email = Email::new(VALID_EMAIL).unwrap();
        let jwt = Jwt::init("HS256", 1, 1, Some("secret"), None, None).unwrap();

        let request = GetAccessTokenUseCaseRequest { email, password, jwt };

        let response = use_case.call(request).await;
        assert!(response.is_err());
        if let Err(e) = response {
            assert_eq!(e, GetAccessTokenError::IncorrectPassword());
        }
    }

    #[tokio::test]
    async fn test_get_access_token_use_case_user_not_found() {
        let user_repository = UserRepositoryMock {};
        let use_case = GetAccessTokenUseCase::new(user_repository);
        let password = Password::new(VALID_PASSWORD, false).unwrap();
        let email = Email::new(EMAIL_NOT_FOUND).unwrap();
        let jwt = Jwt::init("HS256", 1, 1, Some("secret"), None, None).unwrap();

        let request = GetAccessTokenUseCaseRequest { email, password, jwt };

        let response = use_case.call(request).await;
        assert!(response.is_err());
        if let Err(e) = response {
            assert_eq!(e, GetAccessTokenError::UserNotFound());
        }
    }
}
