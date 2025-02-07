//! External link MySQL repository

mod model;

use crate::{
    adapters::database::mysql::{Db, MysqlPagination, MysqlQuerySorts},
    domain::{
        repositories::external_link::{
            dto::{
                CountExternalLinksDtoRequest, CountExternalLinksDtoResponse, CreateExternalLinkDtoRequest,
                CreateExternalLinkDtoResponse, DeleteExternalLinkDtoRequest, DeleteExternalLinkDtoResponse,
                GetExternalLinkByIdDtoRequest, GetExternalLinkByIdDtoResponse, GetExternalLinksDtoRequest,
                GetExternalLinksDtoResponse, RestoreExternalLinkDtoRequest, RestoreExternalLinkDtoResponse,
                UpdateExternalLinkDtoRequest, UpdateExternalLinkDtoResponse,
            },
            ExternalLinkRepository,
        },
        use_cases::external_link::{
            delete_external_link::DeleteExternalLinkUseCaseResponse,
            restore_external_link::RestoreExternalLinkUseCaseResponse,
            update_external_link::UpdateExternalLinkUseCaseResponse, ExternalLinkUseCaseError,
            ExternalLinkUseCaseResponse,
        },
        value_objects::{datetime::UtcDateTime, id::Id},
    },
};
use async_trait::async_trait;
use model::ExternalLinkModel;
use sqlx::Row;
use std::sync::Arc;

/// External link MySQL repository
#[derive(Debug, Clone)]
pub struct ExternalLinkMysqlRepository {
    db: Arc<Db>,
}

impl ExternalLinkMysqlRepository {
    /// Create a new repository
    pub fn new(db: Db) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl ExternalLinkRepository for ExternalLinkMysqlRepository {
    #[instrument(skip(self), name = "external_link_repository_create")]
    async fn create(
        &self,
        req: CreateExternalLinkDtoRequest,
    ) -> Result<CreateExternalLinkDtoResponse, ExternalLinkUseCaseError> {
        let external_link_id = Id::new().map_err(|err| {
            error!(error = %err, "Failed to create external link ID");
            ExternalLinkUseCaseError::InvalidId()
        })?;
        let now = UtcDateTime::now();

        sqlx::query!(
            "
            INSERT INTO external_links (id, name, created_at, updated_at, deleted_at)
            VALUES (?, ?, ?, ?, NULL)
        ",
            external_link_id.clone().to_string(),
            req.0.name,
            now.value(),
            now.value()
        )
        .execute(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to create external link");
            ExternalLinkUseCaseError::DatabaseError("External link creation error".to_string())
        })?;

        Ok(CreateExternalLinkDtoResponse(ExternalLinkUseCaseResponse {
            id: external_link_id,
            name: req.0.name,
            created_at: now.clone(),
            updated_at: now,
            deleted_at: None,
        }))
    }

    #[instrument(skip(self), name = "external_link_repository_count")]
    async fn count_external_links(
        &self,
        req: CountExternalLinksDtoRequest,
    ) -> Result<CountExternalLinksDtoResponse, ExternalLinkUseCaseError> {
        let mut query = String::from("SELECT COUNT(*) AS total FROM external_links");
        query.push_str(match req.deleted {
            true => " WHERE deleted_at IS NOT NULL",
            false => " WHERE deleted_at IS NULL",
        });

        let result = sqlx::query(&query)
            .fetch_one(self.db.pool.clone().as_ref())
            .await
            .map_err(|err| {
                error!(error = %err, "Failed to count external links");
                ExternalLinkUseCaseError::DatabaseError("Failed to count external links".to_string())
            })?;

        Ok(CountExternalLinksDtoResponse(result.try_get("total")?))
    }

    #[instrument(skip(self), name = "external_link_repository_get_all")]
    async fn get_external_links(
        &self,
        req: GetExternalLinksDtoRequest,
    ) -> Result<GetExternalLinksDtoResponse, ExternalLinkUseCaseError> {
        let mut query = String::from(
            r#"
            SELECT id, name, created_at, updated_at, deleted_at
            FROM external_links
        "#,
        );

        query.push_str(match req.0.deleted {
            true => " WHERE deleted_at IS NOT NULL",
            false => " WHERE deleted_at IS NULL",
        });

        // Sorts
        let sorts = MysqlQuerySorts(req.0.sorts.unwrap_or_default());
        query.push_str(&sorts.to_sql(&["name", "created_at", "updated_at", "deleted_at"]));

        // Pagination
        let pagination = MysqlPagination::from(req.0.pagination);
        query.push_str(&pagination.to_sql());

        let external_links = sqlx::query_as::<_, ExternalLinkModel>(&query)
            .fetch_all(self.db.pool.clone().as_ref())
            .await
            .map_err(|err| {
                error!(error = %err, "Failed to get external links");
                ExternalLinkUseCaseError::DatabaseError("Failed to get external links".to_string())
            })?
            .into_iter()
            .map(ExternalLinkUseCaseResponse::try_from)
            .collect::<Result<Vec<ExternalLinkUseCaseResponse>, _>>()
            .map_err(|err| {
                error!(error = %err, "Failed to convert external link model to external link use case response");
                ExternalLinkUseCaseError::FromModelError()
            })?;

        Ok(GetExternalLinksDtoResponse(external_links))
    }

    #[instrument(skip(self), name = "external_link_repository_get_by_id")]
    async fn get_by_id(
        &self,
        req: GetExternalLinkByIdDtoRequest,
    ) -> Result<GetExternalLinkByIdDtoResponse, ExternalLinkUseCaseError> {
        let result = sqlx::query_as!(
            ExternalLinkModel,
            "
            SELECT id, name, created_at, updated_at, deleted_at
            FROM external_links
            WHERE id = ?
                AND deleted_at IS NULL",
            req.0.id.to_string()
        )
        .fetch_optional(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to get external link");
            ExternalLinkUseCaseError::DatabaseError("Failed to get external link".to_string())
        })?;

        let external_link = match result {
            Some(row) => row.try_into().map_err(|err| {
                error!(error = %err, "Failed to convert external_link model to external_link use case response");
                ExternalLinkUseCaseError::FromModelError()
            })?,
            None => Err(ExternalLinkUseCaseError::ExternalLinkNotFound())?,
        };

        Ok(GetExternalLinkByIdDtoResponse(external_link))
    }

    #[instrument(skip(self), name = "external_link_repository_update")]
    async fn update(
        &self,
        req: UpdateExternalLinkDtoRequest,
    ) -> Result<UpdateExternalLinkDtoResponse, ExternalLinkUseCaseError> {
        let result = sqlx::query!(
            "
            UPDATE external_links
            SET name = ?, updated_at = ?
            WHERE id = ?
                AND deleted_at IS NULL",
            req.0.name,
            UtcDateTime::now().value(),
            req.0.id.to_string()
        )
        .execute(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to update external link");
            ExternalLinkUseCaseError::DatabaseError("Failed to update external link".to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(ExternalLinkUseCaseError::ExternalLinkNotFound())?;
        }

        Ok(UpdateExternalLinkDtoResponse(UpdateExternalLinkUseCaseResponse()))
    }

    #[instrument(skip(self), name = "external_link_repository_delete")]
    async fn delete(
        &self,
        req: DeleteExternalLinkDtoRequest,
    ) -> Result<DeleteExternalLinkDtoResponse, ExternalLinkUseCaseError> {
        let result = sqlx::query!(
            "
            UPDATE external_links
            SET deleted_at = ?
            WHERE id = ?
                AND deleted_at IS NULL",
            Some(UtcDateTime::now().value()),
            req.0.id.to_string()
        )
        .execute(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to delete external link");
            ExternalLinkUseCaseError::DatabaseError("Failed to delete external link".to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(ExternalLinkUseCaseError::ExternalLinkNotFound())?;
        }

        Ok(DeleteExternalLinkDtoResponse(DeleteExternalLinkUseCaseResponse()))
    }

    #[instrument(skip(self), name = "external_link_repository_restore")]
    async fn restore(
        &self,
        req: RestoreExternalLinkDtoRequest,
    ) -> Result<RestoreExternalLinkDtoResponse, ExternalLinkUseCaseError> {
        let result = sqlx::query!(
            "
            UPDATE external_links
            SET deleted_at = NULL
            WHERE id = ?
                AND deleted_at IS NOT NULL",
            req.0.id.to_string()
        )
        .execute(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to restore external link");
            ExternalLinkUseCaseError::DatabaseError("Failed to restore external link".to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(ExternalLinkUseCaseError::ExternalLinkNotFound())?;
        }

        Ok(RestoreExternalLinkDtoResponse(RestoreExternalLinkUseCaseResponse()))
    }
}
