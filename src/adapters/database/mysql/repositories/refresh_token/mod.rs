//! Refresh token MySQL repository

use crate::adapters::database::mysql::Db;
use crate::domain::repositories::refresh_token::dto::{
    CreateRefreshTokenDtoRequest, CreateRefreshTokenDtoResponse, DeleteRefreshTokenDtoRequest,
    DeleteRefreshTokenDtoResponse, GetRefreshTokenDtoRequest, GetRefreshTokenDtoResponse,
};
use crate::domain::repositories::refresh_token::RefreshTokenRepository;
use crate::domain::use_cases::user::UserUseCaseError;
use async_trait::async_trait;
use std::sync::Arc;

/// Refresh token MySQL repository
#[derive(Debug, Clone)]
pub struct RefreshTokenMysqlRepository {
    db: Arc<Db>,
}

impl RefreshTokenMysqlRepository {
    /// Create a new repository
    pub fn new(db: Db) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl RefreshTokenRepository for RefreshTokenMysqlRepository {
    #[instrument(skip(self), name = "refresh_token_repository_create")]
    async fn create_refresh_token(
        &self,
        req: CreateRefreshTokenDtoRequest,
    ) -> Result<CreateRefreshTokenDtoResponse, UserUseCaseError> {
        sqlx::query!(
            r#"
                INSERT INTO refresh_tokens (refresh_token, user_id, access_token, expired_at)
                VALUES (?, ?, ?, ?)
            "#,
            req.refresh_token.refresh_token.to_string(),
            req.user_id.to_string(),
            req.access_token.token.to_string(),
            req.refresh_token.expired_at.value(),
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        Ok(CreateRefreshTokenDtoResponse())
    }

    /// Get a refresh token
    #[instrument(skip(self), name = "refresh_token_repository_get")]
    async fn get_refresh_token(
        &self,
        _req: GetRefreshTokenDtoRequest,
    ) -> Result<GetRefreshTokenDtoResponse, UserUseCaseError> {
        todo!()
    }

    /// Delete a refresh token
    #[instrument(skip(self), name = "refresh_token_repository_delete")]
    async fn delete_refresh_token(
        &self,
        _req: DeleteRefreshTokenDtoRequest,
    ) -> Result<DeleteRefreshTokenDtoResponse, UserUseCaseError> {
        todo!()
    }
}
