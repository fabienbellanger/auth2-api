//! User refresh token use case

use super::UserUseCaseError;
use crate::domain::{
    entities::{access_token::AccessToken, refresh_token::RefreshToken},
    repositories::refresh_token::{dto::GetRefreshTokenDtoRequest, RefreshTokenRepository},
    services::security::jwt::Jwt,
};

#[derive(Debug, Clone)]
pub struct RefreshTokenUseCaseRequest {
    pub refresh_token: RefreshToken,
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
    #[instrument(skip(self), name = "get_users_use_case")]
    pub async fn call(
        &self,
        request: RefreshTokenUseCaseRequest,
    ) -> Result<RefreshTokenUseCaseResponse, UserUseCaseError> {
        // Get the refresh token
        let _refresh_token = self
            .refresh_token_repository
            .get_refresh_token(GetRefreshTokenDtoRequest(request.refresh_token))
            .await?;

        // Delete the refresh token

        // Generate a new access

        // Generate and save a new refresh token

        todo!()
    }
}
