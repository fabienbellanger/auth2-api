//! Refresh token repository

pub mod dto;

use crate::domain::repositories::refresh_token::dto::{
    DeleteExpiredRefreshTokensDtoRequest, DeleteExpiredRefreshTokensDtoResponse,
};
use crate::domain::use_cases::user::UserUseCaseError;
use async_trait::async_trait;
use dto::{
    CreateRefreshTokenDtoRequest, CreateRefreshTokenDtoResponse, DeleteRefreshTokenDtoRequest,
    DeleteRefreshTokenDtoResponse, GetRefreshTokenDtoRequest, GetRefreshTokenDtoResponse,
};

#[async_trait]
pub trait RefreshTokenRepository: Clone {
    /// Create a new refresh token
    async fn create_refresh_token(
        &self,
        req: CreateRefreshTokenDtoRequest,
    ) -> Result<CreateRefreshTokenDtoResponse, UserUseCaseError>;

    /// Get a refresh token
    async fn get_refresh_token(
        &self,
        req: GetRefreshTokenDtoRequest,
    ) -> Result<GetRefreshTokenDtoResponse, UserUseCaseError>;

    /// Delete a refresh token
    async fn delete_refresh_token(
        &self,
        req: DeleteRefreshTokenDtoRequest,
    ) -> Result<DeleteRefreshTokenDtoResponse, UserUseCaseError>;

    /// Delete expired refresh tokens
    async fn delete_expired_refresh_tokens(
        &self,
        req: DeleteExpiredRefreshTokensDtoRequest,
    ) -> Result<DeleteExpiredRefreshTokensDtoResponse, UserUseCaseError>;
}
