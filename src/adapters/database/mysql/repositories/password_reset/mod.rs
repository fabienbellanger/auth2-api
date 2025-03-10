//! Password reset MySQL repository

use crate::adapters::database::mysql::Db;
use crate::domain::repositories::password_reset::PasswordResetRepository;
use crate::domain::repositories::password_reset::dto::{
    CreateUpdatePasswordResetDtoRequest, CreateUpdatePasswordResetDtoResponse, DeletePasswordResetDtoRequest,
    DeletePasswordResetDtoResponse, GetUserIdFromTokenDtoRequest, GetUserIdFromTokenDtoResponse,
};
use crate::domain::use_cases::user::UserUseCaseError;
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::id::Id;
use async_trait::async_trait;
use std::str::FromStr;
use std::sync::Arc;

/// Password reset MySQL repository
#[derive(Debug, Clone)]
pub struct PasswordResetMysqlRepository {
    db: Arc<Db>,
}

impl PasswordResetMysqlRepository {
    /// Create a new repository
    pub fn new(db: Db) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl PasswordResetRepository for PasswordResetMysqlRepository {
    #[instrument(skip(self), name = "password_reset_repository_create_update")]
    async fn create_or_update(
        &self,
        req: CreateUpdatePasswordResetDtoRequest,
    ) -> Result<CreateUpdatePasswordResetDtoResponse, UserUseCaseError> {
        sqlx::query!(
            r#"
                INSERT INTO password_resets (user_id, token, expired_at)
                VALUES (?, ?, ?)
                ON DUPLICATE KEY UPDATE token = ?, expired_at = ?
            "#,
            req.0.user_id.to_string(),
            req.0.token,
            req.0.expired_at.value(),
            req.0.token,
            req.0.expired_at.value(),
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        Ok(CreateUpdatePasswordResetDtoResponse())
    }

    #[instrument(skip(self), name = "password_reset_repository_get_user")]
    async fn get_user_from_token(
        &self,
        req: GetUserIdFromTokenDtoRequest,
    ) -> Result<GetUserIdFromTokenDtoResponse, UserUseCaseError> {
        let result = sqlx::query!(
            r#"
                SELECT u.id AS user_id
                FROM password_resets pr
                    INNER JOIN users u ON u.id = pr.user_id AND u.deleted_at IS NULL
                WHERE pr.token = ?
                    AND pr.expired_at >= ?
                LIMIT 1
            "#,
            req.token,
            UtcDateTime::now().value(),
        )
        .fetch_optional(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to get user ID from password reset token");
            UserUseCaseError::DatabaseError("Failed to get user ID from password reset token".to_string())
        })?;

        let user_id = match result {
            Some(r) => Some(Id::from_str(&r.user_id)?),
            None => None,
        };

        Ok(GetUserIdFromTokenDtoResponse { user_id })
    }

    #[instrument(skip(self), name = "password_reset_repository_delete")]
    async fn delete(
        &self,
        req: DeletePasswordResetDtoRequest,
    ) -> Result<DeletePasswordResetDtoResponse, UserUseCaseError> {
        sqlx::query!(
            r#"
                DELETE FROM password_resets
                WHERE user_id = ?
            "#,
            req.user_id.to_string(),
        )
        .execute(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to delete password reset");
            UserUseCaseError::DatabaseError("Failed to delete password reset".to_string())
        })?;

        Ok(DeletePasswordResetDtoResponse())
    }
}
