//! User refresh token use case

use super::UserUseCaseError;
use crate::domain::entities::refresh_token::RefreshTokenId;
use crate::domain::repositories::refresh_token::dto::{CreateRefreshTokenDtoRequest, DeleteRefreshTokenDtoRequest};
use crate::domain::services::security::payload::PayloadData;
use crate::domain::{
    entities::{access_token::AccessToken, refresh_token::RefreshToken},
    repositories::refresh_token::{dto::GetRefreshTokenDtoRequest, RefreshTokenRepository},
    services::security::jwt::Jwt,
};

#[derive(Debug, Clone)]
pub struct RefreshTokenUseCaseRequest {
    pub refresh_token_id: RefreshTokenId,
    pub jwt: Jwt,
}

#[derive(Debug, Clone)]
pub struct RefreshTokenUseCaseResponse {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

#[derive(Debug, Clone)]
pub struct RefreshTokenUseCase<T: RefreshTokenRepository> {
    refresh_token_repository: T,
}

impl<T: RefreshTokenRepository> RefreshTokenUseCase<T> {
    /// Create a new use case
    pub fn new(refresh_token_repository: T) -> Self {
        Self {
            refresh_token_repository,
        }
    }

    /// Get all users
    #[instrument(skip(self), name = "refresh_token_use_case")]
    pub async fn call(
        &self,
        request: RefreshTokenUseCaseRequest,
    ) -> Result<RefreshTokenUseCaseResponse, UserUseCaseError> {
        // Get the refresh token
        let refresh_token_resp = self
            .refresh_token_repository
            .get_refresh_token(GetRefreshTokenDtoRequest(request.refresh_token_id.clone()))
            .await?;
        let user_id = refresh_token_resp.user_id;

        // Delete the refresh token
        self.refresh_token_repository
            .delete_refresh_token(DeleteRefreshTokenDtoRequest(request.refresh_token_id))
            .await?;

        // Generate a new access token
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

        Ok(RefreshTokenUseCaseResponse {
            access_token,
            refresh_token,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::mock::refresh_token::{
        RefreshTokenRepositoryMock, INVALID_REFRESH_TOKEN_ID, VALID_REFRESH_TOKEN_ID,
    };
    use crate::domain::value_objects::id::Id;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_refresh_token_use_case() {
        let refresh_token_repository = RefreshTokenRepositoryMock {};
        let use_case = RefreshTokenUseCase::new(refresh_token_repository);
        let jwt = Jwt::init("HS256", 1, 1, Some("secret"), None, None).unwrap();
        let refresh_token_id = Id::from_str(VALID_REFRESH_TOKEN_ID).unwrap();

        let response = use_case
            .call(RefreshTokenUseCaseRequest {
                refresh_token_id: refresh_token_id,
                jwt,
            })
            .await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_refresh_token_use_case_invalid_token() {
        let refresh_token_repository = RefreshTokenRepositoryMock {};
        let use_case = RefreshTokenUseCase::new(refresh_token_repository);
        let jwt = Jwt::init("HS256", 1, 1, Some("secret"), None, None).unwrap();
        let refresh_token_id = Id::from_str(INVALID_REFRESH_TOKEN_ID).unwrap();

        let response = use_case
            .call(RefreshTokenUseCaseRequest {
                refresh_token_id: refresh_token_id,
                jwt,
            })
            .await;
        assert!(response.is_err());
    }
}
