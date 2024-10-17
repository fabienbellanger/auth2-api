//! Scope MySQL repository

use crate::adapters::database::mysql::repositories::scope::model::ScopeModel;
use crate::adapters::database::mysql::{Db, MysqlPagination, MysqlQuerySorts};
use crate::domain::repositories::scope::dto::{
    CountScopesDtoRequest, CountScopesDtoResponse, CreateScopeDtoRequest, CreateScopeDtoResponse,
    DeleteScopeDtoRequest, DeleteScopeDtoResponse, GetScopesDtoRequest, GetScopesDtoResponse, RestoreScopeDtoRequest,
    RestoreScopeDtoResponse,
};
use crate::domain::repositories::scope::ScopeRepository;
use crate::domain::use_cases::scope::delete_scope::DeleteScopeUseCaseResponse;
use crate::domain::use_cases::scope::restore_scope::RestoreScopeUseCaseResponse;
use crate::domain::use_cases::scope::{ScopeUseCaseError, ScopeUseCaseResponse};
use crate::domain::value_objects::datetime::UtcDateTime;
use async_trait::async_trait;
use sqlx::Row;
use std::sync::Arc;

mod model;

/// Scope MySQL repository
#[derive(Debug, Clone)]
pub struct ScopeMysqlRepository {
    db: Arc<Db>,
}

impl ScopeMysqlRepository {
    /// Create a new repository
    pub fn new(db: Db) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl ScopeRepository for ScopeMysqlRepository {
    #[instrument(skip(self), name = "scope_repository_create")]
    async fn create(&self, req: CreateScopeDtoRequest) -> Result<CreateScopeDtoResponse, ScopeUseCaseError> {
        let now = UtcDateTime::now();

        sqlx::query!(
            "
            INSERT INTO scopes (id, application_id, created_at, updated_at, deleted_at)
            VALUES (?, ?, ?, ?, NULL)
        ",
            req.0.id.to_string(),
            req.0.application_id.to_string(),
            now.value(),
            now.value()
        )
        .execute(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to create scope");
            match err {
                sqlx::Error::Database(err) if err.is_unique_violation() => ScopeUseCaseError::IdAlreadyExists(),
                _ => ScopeUseCaseError::DatabaseError("Scope creation error".to_string()),
            }
        })?;

        Ok(CreateScopeDtoResponse(ScopeUseCaseResponse {
            id: req.0.id,
            application_id: req.0.application_id,
            created_at: now.clone(),
            updated_at: now,
            deleted_at: None,
        }))
    }

    #[instrument(skip(self), name = "scope_repository_get_all")]
    async fn get_scopes(&self, req: GetScopesDtoRequest) -> Result<GetScopesDtoResponse, ScopeUseCaseError> {
        let mut query = String::from(
            r#"
            SELECT id, application_id, created_at, updated_at, deleted_at
            FROM scopes
        "#,
        );

        query.push_str(match req.0.deleted {
            true => "WHERE deleted_at IS NOT NULL",
            false => "WHERE deleted_at IS NULL",
        });

        if let Some(application_id) = req.0.application_id {
            query.push_str(&format!(r#" AND application_id = "{application_id}""#));
        }

        // Sorts
        let sorts = MysqlQuerySorts(req.0.sorts.unwrap_or_default());
        query.push_str(&sorts.to_sql(&["id", "created_at", "updated_at", "deleted_at"]));

        // Pagination
        let pagination = MysqlPagination::from(req.0.pagination);
        query.push_str(&pagination.to_sql());

        let scopes = sqlx::query_as::<_, ScopeModel>(&query)
            .fetch_all(self.db.pool.clone().as_ref())
            .await
            .map_err(|err| {
                error!(error = %err, "Failed to get scopes");
                ScopeUseCaseError::DatabaseError("Failed to get scopes".to_string())
            })?
            .into_iter()
            .map(ScopeUseCaseResponse::try_from)
            .collect::<Result<Vec<ScopeUseCaseResponse>, _>>()
            .map_err(|err| {
                error!(error = %err, "Failed to convert scope model to scope use case response");
                ScopeUseCaseError::FromModelError()
            })?;

        Ok(GetScopesDtoResponse(scopes))
    }

    #[instrument(skip(self), name = "scope_repository_count_all")]
    async fn count_scopes(&self, req: CountScopesDtoRequest) -> Result<CountScopesDtoResponse, ScopeUseCaseError> {
        let mut query = String::from("SELECT COUNT(*) AS total FROM scopes");
        query.push_str(match req.deleted {
            true => " WHERE deleted_at IS NOT NULL",
            false => " WHERE deleted_at IS NULL",
        });

        if let Some(application_id) = req.application_id {
            query.push_str(&format!(r#" AND application_id = "{application_id}""#));
        }

        let result = sqlx::query(&query)
            .fetch_one(self.db.pool.clone().as_ref())
            .await
            .map_err(|err| {
                error!(error = %err, "Failed to count scopes");
                ScopeUseCaseError::DatabaseError("Failed to count scopes".to_string())
            })?;

        Ok(CountScopesDtoResponse(result.try_get("total")?))
    }

    #[instrument(skip(self), name = "scope_repository_delete")]
    async fn delete(&self, req: DeleteScopeDtoRequest) -> Result<DeleteScopeDtoResponse, ScopeUseCaseError> {
        let result = sqlx::query!(
            "
            UPDATE scopes
            SET deleted_at = ?
            WHERE id = ?
                AND deleted_at IS NULL",
            Some(UtcDateTime::now().value()),
            req.0.id.to_string()
        )
        .execute(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to delete scope");
            ScopeUseCaseError::DatabaseError("Failed to delete scope".to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(ScopeUseCaseError::ScopeNotFound())?;
        }

        Ok(DeleteScopeDtoResponse(DeleteScopeUseCaseResponse()))
    }

    #[instrument(skip(self), name = "scope_repository_restore")]
    async fn restore(&self, req: RestoreScopeDtoRequest) -> Result<RestoreScopeDtoResponse, ScopeUseCaseError> {
        let result = sqlx::query!(
            "
            UPDATE scopes
            SET deleted_at = NULL
            WHERE id = ?
                AND deleted_at IS NOT NULL",
            req.0.id.to_string()
        )
        .execute(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to restore scope");
            ScopeUseCaseError::DatabaseError("Failed to restore scope".to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(ScopeUseCaseError::ScopeNotFound())?;
        }

        Ok(RestoreScopeDtoResponse(RestoreScopeUseCaseResponse()))
    }
}
