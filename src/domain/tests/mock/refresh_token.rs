//! Mock of the refresh token repository

use crate::domain::repositories::refresh_token::dto::{
    CreateRefreshTokenDtoRequest, CreateRefreshTokenDtoResponse, DeleteRefreshTokenDtoRequest,
    DeleteRefreshTokenDtoResponse, GetRefreshTokenDtoRequest, GetRefreshTokenDtoResponse,
};
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

    /// Get a refresh token
    async fn get_refresh_token(
        &self,
        _req: GetRefreshTokenDtoRequest,
    ) -> Result<GetRefreshTokenDtoResponse, UserUseCaseError> {
        todo!()
    }

    /// Delete a refresh token
    async fn delete_refresh_token(
        &self,
        _req: DeleteRefreshTokenDtoRequest,
    ) -> Result<DeleteRefreshTokenDtoResponse, UserUseCaseError> {
        todo!()
    }
}
