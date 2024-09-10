//! Clean expired refresh token use case

//! User refresh token use case

use crate::domain::repositories::refresh_token::dto::DeleteExpiredRefreshTokensDtoRequest;
use crate::domain::repositories::refresh_token::RefreshTokenRepository;
use crate::domain::use_cases::user::UserUseCaseError;

#[derive(Debug, Clone)]
pub struct CleanExpiredRefreshTokensUseCaseRequest();

#[derive(Debug, Clone)]
pub struct CleanExpiredRefreshTokensUseCaseResponse {
    pub deleted: u64,
}

#[derive(Debug, Clone)]
pub struct CleanExpiredRefreshTokens<T: RefreshTokenRepository> {
    refresh_token_repository: T,
}

impl<T: RefreshTokenRepository> CleanExpiredRefreshTokens<T> {
    /// Create a new use case
    pub fn new(refresh_token_repository: T) -> Self {
        Self {
            refresh_token_repository,
        }
    }

    /// Delete expired refresh tokens
    #[instrument(skip(self), name = "clean_expired_refresh_tokens_use_case")]
    pub async fn call(
        &self,
        _request: CleanExpiredRefreshTokensUseCaseRequest,
    ) -> Result<CleanExpiredRefreshTokensUseCaseResponse, UserUseCaseError> {
        let response = self
            .refresh_token_repository
            .delete_expired_refresh_tokens(DeleteExpiredRefreshTokensDtoRequest())
            .await?;

        Ok(CleanExpiredRefreshTokensUseCaseResponse {
            deleted: response.deleted,
        })
    }
}
