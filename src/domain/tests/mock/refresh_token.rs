//! Mock of the refresh token repository

use crate::domain::repositories::refresh_token::dto::{
    CreateRefreshTokenDtoRequest, CreateRefreshTokenDtoResponse, DeleteExpiredRefreshTokensDtoRequest,
    DeleteExpiredRefreshTokensDtoResponse, DeleteRefreshTokenDtoRequest, DeleteRefreshTokenDtoResponse,
    GetRefreshTokenDtoRequest, GetRefreshTokenDtoResponse,
};
use crate::domain::repositories::refresh_token::RefreshTokenRepository;
use crate::domain::use_cases::user::UserUseCaseError;
use crate::domain::value_objects::id::Id;
use async_trait::async_trait;
use std::str::FromStr;

pub const VALID_REFRESH_TOKEN_ID: &str = "3288fb86-db99-471d-95bc-1451c7ec6f7b";
pub const INVALID_REFRESH_TOKEN_ID: &str = "3288fb86-db99-471d-95bc-1451c7ec6f7a";

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
        req: GetRefreshTokenDtoRequest,
    ) -> Result<GetRefreshTokenDtoResponse, UserUseCaseError> {
        let token_id = req.0;

        if token_id == Id::from_str(VALID_REFRESH_TOKEN_ID)? {
            return Ok(GetRefreshTokenDtoResponse { user_id: Id::new()? });
        }

        Err(UserUseCaseError::RefreshTokenCreationError(
            "Refresh token creation error: ".to_string(),
        ))
    }

    /// Delete a refresh token
    async fn delete_refresh_token(
        &self,
        req: DeleteRefreshTokenDtoRequest,
    ) -> Result<DeleteRefreshTokenDtoResponse, UserUseCaseError> {
        let token_id = req.0;

        if token_id == Id::from_str(VALID_REFRESH_TOKEN_ID)? {
            return Ok(DeleteRefreshTokenDtoResponse());
        }

        Err(UserUseCaseError::RefreshTokenCreationError(
            "Refresh token creation error".to_string(),
        ))
    }

    /// Delete expired refresh tokens
    async fn delete_expired_refresh_tokens(
        &self,
        _req: DeleteExpiredRefreshTokensDtoRequest,
    ) -> Result<DeleteExpiredRefreshTokensDtoResponse, UserUseCaseError> {
        Ok(DeleteExpiredRefreshTokensDtoResponse { deleted: 23 })
    }
}
