//! Mock of the refresh token repository

use crate::domain::repositories::refresh_token::dto::{CreateRefreshTokenDtoRequest, CreateRefreshTokenDtoResponse};
use crate::domain::repositories::refresh_token::RefreshTokenRepository;
use crate::domain::use_cases::user::UserUseCaseError;
use async_trait::async_trait;

/// Refresh token repository mock
#[derive(Debug, Clone)]
pub struct RefreshTokenRepositoryMock {}

#[async_trait]
impl RefreshTokenRepository for RefreshTokenRepositoryMock {
    /// Create a new refresh token
    async fn create_refresh_token(
        &self,
        _req: CreateRefreshTokenDtoRequest,
    ) -> Result<CreateRefreshTokenDtoResponse, UserUseCaseError> {
        Ok(CreateRefreshTokenDtoResponse())
    }
}
