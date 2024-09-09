//! Refresh token MySQL repository

use crate::adapters::database::mysql::Db;
use crate::domain::repositories::refresh_token::dto::{
    CreateRefreshTokenDtoRequest, CreateRefreshTokenDtoResponse, DeleteRefreshTokenDtoRequest,
    DeleteRefreshTokenDtoResponse, GetRefreshTokenDtoRequest, GetRefreshTokenDtoResponse,
};
use crate::domain::repositories::refresh_token::RefreshTokenRepository;
use crate::domain::use_cases::user::UserUseCaseError;
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::id::Id;
use async_trait::async_trait;
use std::str::FromStr;
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
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to create refresh token");
            UserUseCaseError::DatabaseError("Failed to create refresh token".to_string())
        })?;

        Ok(CreateRefreshTokenDtoResponse())
    }

    /// Get a refresh token
    #[instrument(skip(self), name = "refresh_token_repository_get")]
    async fn get_refresh_token(
        &self,
        req: GetRefreshTokenDtoRequest,
    ) -> Result<GetRefreshTokenDtoResponse, UserUseCaseError> {
        let row = sqlx::query!(
            r#"
                SELECT refresh_token, user_id
                FROM refresh_tokens 
                WHERE refresh_token = ? 
                    AND expired_at >= ?
            "#,
            req.0.to_string(),
            UtcDateTime::now().value(),
        )
        .fetch_optional(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to get refresh token");
            UserUseCaseError::DatabaseError("Failed to get refresh token".to_string())
        })?;

        let response = match row {
            Some(row) => {
                let user_id = Id::from_str(&row.user_id)?;
                GetRefreshTokenDtoResponse { user_id }
            }
            None => Err(UserUseCaseError::InvalidRefreshToken())?,
        };

        Ok(response)
    }

    /// Delete a refresh token
    #[instrument(skip(self), name = "refresh_token_repository_delete")]
    async fn delete_refresh_token(
        &self,
        req: DeleteRefreshTokenDtoRequest,
    ) -> Result<DeleteRefreshTokenDtoResponse, UserUseCaseError> {
        let _result = sqlx::query!(
            r#"
                DELETE FROM refresh_tokens 
                WHERE refresh_token = ?
            "#,
            req.0.to_string(),
        )
        .execute(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to delete refresh token");
            UserUseCaseError::DatabaseError("Failed to delete refresh token".to_string())
        })?;

        Ok(DeleteRefreshTokenDtoResponse())
    }
}
