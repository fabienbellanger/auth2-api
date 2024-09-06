//! Refresh token repository

pub mod dto;

use crate::domain::repositories::refresh_token::dto::{CreateRefreshTokenDtoRequest, CreateRefreshTokenDtoResponse};
use crate::domain::use_cases::user::UserUseCaseError;
use async_trait::async_trait;

#[async_trait]
pub trait RefreshTokenRepository: Clone {
    /// Create a new refresh token
    async fn create_refresh_token(
        &self,
        req: CreateRefreshTokenDtoRequest,
    ) -> Result<CreateRefreshTokenDtoResponse, UserUseCaseError>;
}
