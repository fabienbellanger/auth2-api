//! Scope MySQL repository

use crate::adapters::database::mysql::Db;
use crate::domain::repositories::scope::dto::{CreateScopeDtoRequest, CreateScopeDtoResponse};
use crate::domain::repositories::scope::ScopeRepository;
use crate::domain::use_cases::scope::{ScopeUseCaseError, ScopeUseCaseResponse};
use crate::domain::value_objects::datetime::UtcDateTime;
use async_trait::async_trait;
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
}
