//! Get Access Token Use Case

use crate::domain::entities::access_token::AccessToken;
use crate::domain::entities::refresh_token::RefreshToken;
use crate::domain::repositories::refresh_token::dto::CreateRefreshTokenDtoRequest;
use crate::domain::repositories::refresh_token::RefreshTokenRepository;
use crate::domain::repositories::user::dto::GetAccessTokenInformationDtoRequest;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::security::jwt::Jwt;
use crate::domain::services::security::payload::PayloadData;
use crate::domain::use_cases::user::UserUseCaseError;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;

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

#[derive(Debug, Clone)]
pub struct GetAccessTokenUseCaseResponse {
    /// Access token
    pub access_token: AccessToken,

    /// Refresh token
    pub refresh_token: RefreshToken,
}

#[derive(Debug, Clone)]
pub struct GetAccessTokenUseCase<U: UserRepository, T: RefreshTokenRepository> {
    user_repository: U,
    refresh_token_repository: T,
}

impl<U: UserRepository, T: RefreshTokenRepository> GetAccessTokenUseCase<U, T> {
    /// Create a new use case
    pub fn new(user_repository: U, refresh_token_repository: T) -> Self {
        Self {
            user_repository,
            refresh_token_repository,
        }
    }

    /// Generate a new access token
    #[instrument(skip(self), name = "get_access_token_use_case")]
    pub async fn call(
        &self,
        request: GetAccessTokenUseCaseRequest,
    ) -> Result<GetAccessTokenUseCaseResponse, UserUseCaseError> {
        // TODO: Validation?

        let original_password = request.password.original().ok_or(UserUseCaseError::InvalidPassword(
            "No hashed password not found".to_string(),
        ))?;

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
                    Err(UserUseCaseError::IncorrectPassword())?
                }
            }
            None => Err(UserUseCaseError::Unauthorized())?,
        };

        // Generate access token
        let payload = PayloadData::new(user_id.to_string(), "".to_string(), "".to_string());
        let access_token = request.jwt.generate(payload).map_err(|err| {
            error!(error = %err, "Error generating access token");
            UserUseCaseError::AccessTokenGenerationError()
        })?;

        // Generate and save refresh token
        let refresh_token =
            RefreshToken::create(user_id.clone(), access_token.clone(), request.jwt.refresh_lifetime())?;
        self.refresh_token_repository
            .create_refresh_token(CreateRefreshTokenDtoRequest {
                refresh_token: refresh_token.clone(),
                access_token: access_token.clone(),
                user_id,
            })
            .await?;

        Ok(GetAccessTokenUseCaseResponse {
            access_token,
            refresh_token,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::mock::refresh_token::RefreshTokenRepositoryMock;
    use crate::domain::tests::mock::user::{
        UserRepositoryMock, EMAIL_NOT_FOUND, INVALID_EMAIL, INVALID_PASSWORD, VALID_EMAIL, VALID_PASSWORD,
    };
    use crate::domain::use_cases::user::UserUseCaseError;

    #[tokio::test]
    async fn test_get_access_token_use_case() {
        let user_repository = UserRepositoryMock {};
        let refresh_token_repository = RefreshTokenRepositoryMock {};
        let use_case = GetAccessTokenUseCase::new(user_repository, refresh_token_repository);
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
        let refresh_token_repository = RefreshTokenRepositoryMock {};
        let use_case = GetAccessTokenUseCase::new(user_repository, refresh_token_repository);
        let password = Password::new(VALID_PASSWORD, false).unwrap();
        let email = Email::new(INVALID_EMAIL).unwrap();
        let jwt = Jwt::init("HS256", 1, 1, Some("secret"), None, None).unwrap();

        let request = GetAccessTokenUseCaseRequest { email, password, jwt };

        let response = use_case.call(request).await;
        assert!(response.is_err());
        if let Err(e) = response {
            assert_eq!(e, UserUseCaseError::DatabaseError("User not found".to_string()));
        }
    }

    #[tokio::test]
    async fn test_get_access_token_use_case_incorrect_password() {
        let user_repository = UserRepositoryMock {};
        let refresh_token_repository = RefreshTokenRepositoryMock {};
        let use_case = GetAccessTokenUseCase::new(user_repository, refresh_token_repository);
        let password = Password::new(INVALID_PASSWORD, false).unwrap();
        let email = Email::new(VALID_EMAIL).unwrap();
        let jwt = Jwt::init("HS256", 1, 1, Some("secret"), None, None).unwrap();

        let request = GetAccessTokenUseCaseRequest { email, password, jwt };

        let response = use_case.call(request).await;
        assert!(response.is_err());
        if let Err(e) = response {
            assert_eq!(e, UserUseCaseError::IncorrectPassword());
        }
    }

    #[tokio::test]
    async fn test_get_access_token_use_case_user_not_found() {
        let user_repository = UserRepositoryMock {};
        let refresh_token_repository = RefreshTokenRepositoryMock {};
        let use_case = GetAccessTokenUseCase::new(user_repository, refresh_token_repository);
        let password = Password::new(VALID_PASSWORD, false).unwrap();
        let email = Email::new(EMAIL_NOT_FOUND).unwrap();
        let jwt = Jwt::init("HS256", 1, 1, Some("secret"), None, None).unwrap();

        let request = GetAccessTokenUseCaseRequest { email, password, jwt };

        let response = use_case.call(request).await;
        assert!(response.is_err());
        if let Err(e) = response {
            assert_eq!(e, UserUseCaseError::Unauthorized());
        }
    }
}
