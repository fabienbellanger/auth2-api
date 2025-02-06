//! Application MySQL repository

mod model;

use crate::adapters::database::mysql::repositories::application::model::ApplicationModel;
use crate::adapters::database::mysql::{Db, MysqlPagination, MysqlQuerySorts};
use crate::domain::repositories::application::dto::{
    CountApplicationsDtoRequest, CountApplicationsDtoResponse, CreateApplicationDtoRequest,
    CreateApplicationDtoResponse, DeleteApplicationDtoRequest, DeleteApplicationDtoResponse,
    GetApplicationByIdDtoRequest, GetApplicationByIdDtoResponse, GetApplicationsDtoRequest, GetApplicationsDtoResponse,
    RestoreApplicationDtoRequest, RestoreApplicationDtoResponse, UpdateApplicationDtoRequest,
    UpdateApplicationDtoResponse,
};
use crate::domain::repositories::application::ApplicationRepository;
use crate::domain::use_cases::application::delete_application::DeleteApplicationUseCaseResponse;
use crate::domain::use_cases::application::restore_application::RestoreApplicationUseCaseResponse;
use crate::domain::use_cases::application::update_application::UpdateApplicationUseCaseResponse;
use crate::domain::use_cases::application::{ApplicationUseCaseError, ApplicationUseCaseResponse};
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::id::Id;
use async_trait::async_trait;
use sqlx::Row;
use std::sync::Arc;

/// Application MySQL repository
#[derive(Debug, Clone)]
pub struct ApplicationMysqlRepository {
    db: Arc<Db>,
}

impl ApplicationMysqlRepository {
    /// Create a new repository
    pub fn new(db: Db) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl ApplicationRepository for ApplicationMysqlRepository {
    #[instrument(skip(self), name = "application_repository_create")]
    async fn create(
        &self,
        req: CreateApplicationDtoRequest,
    ) -> Result<CreateApplicationDtoResponse, ApplicationUseCaseError> {
        let application_id = Id::new().map_err(|err| {
            error!(error = %err, "Failed to create application ID");
            ApplicationUseCaseError::InvalidId()
        })?;
        let now = UtcDateTime::now();

        sqlx::query!(
            "
            INSERT INTO applications (id, name, created_at, updated_at, deleted_at)
            VALUES (?, ?, ?, ?, NULL)
        ",
            application_id.clone().to_string(),
            req.0.name,
            now.value(),
            now.value()
        )
        .execute(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to create application");
            ApplicationUseCaseError::DatabaseError("Application creation error".to_string())
        })?;

        Ok(CreateApplicationDtoResponse(ApplicationUseCaseResponse {
            id: application_id,
            name: req.0.name,
            created_at: now.clone(),
            updated_at: now,
            deleted_at: None,
        }))
    }

    #[instrument(skip(self), name = "application_repository_get_by_id")]
    async fn get_by_id(
        &self,
        req: GetApplicationByIdDtoRequest,
    ) -> Result<GetApplicationByIdDtoResponse, ApplicationUseCaseError> {
        let result = sqlx::query_as!(
            ApplicationModel,
            "
            SELECT id, name, created_at, updated_at, deleted_at
            FROM applications
            WHERE id = ?
                AND deleted_at IS NULL",
            req.0.id.to_string()
        )
        .fetch_optional(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to get application");
            ApplicationUseCaseError::DatabaseError("Failed to get application".to_string())
        })?;

        let application = match result {
            Some(row) => row.try_into().map_err(|err| {
                error!(error = %err, "Failed to convert application model to application use case response");
                ApplicationUseCaseError::FromModelError()
            })?,
            None => Err(ApplicationUseCaseError::ApplicationNotFound())?,
        };

        Ok(GetApplicationByIdDtoResponse(application))
    }

    #[instrument(skip(self), name = "application_repository_get_all")]
    async fn get_applications(
        &self,
        req: GetApplicationsDtoRequest,
    ) -> Result<GetApplicationsDtoResponse, ApplicationUseCaseError> {
        let mut query = String::from(
            r#"
            SELECT id, name, created_at, updated_at, deleted_at
            FROM applications
        "#,
        );

        query.push_str(match req.0.deleted {
            true => " WHERE deleted_at IS NOT NULL",
            false => " WHERE deleted_at IS NULL",
        });

        // Sorts
        let sorts = MysqlQuerySorts(req.0.sorts.unwrap_or_default());
        query.push_str(&sorts.to_sql(&["id", "name", "updated_at", "deleted_at"]));

        // Pagination
        let pagination = MysqlPagination::from(req.0.pagination);
        query.push_str(&pagination.to_sql());

        let applications = sqlx::query_as::<_, ApplicationModel>(&query)
            .fetch_all(self.db.pool.clone().as_ref())
            .await
            .map_err(|err| {
                error!(error = %err, "Failed to get applications");
                ApplicationUseCaseError::DatabaseError("Failed to get applications".to_string())
            })?
            .into_iter()
            .map(ApplicationUseCaseResponse::try_from)
            .collect::<Result<Vec<ApplicationUseCaseResponse>, _>>()
            .map_err(|err| {
                error!(error = %err, "Failed to convert application model to application use case response");
                ApplicationUseCaseError::FromModelError()
            })?;

        Ok(GetApplicationsDtoResponse(applications))
    }

    #[instrument(skip(self), name = "application_repository_update")]
    async fn update(
        &self,
        req: UpdateApplicationDtoRequest,
    ) -> Result<UpdateApplicationDtoResponse, ApplicationUseCaseError> {
        let result = sqlx::query!(
            "
            UPDATE applications
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
            error!(error = %err, "Failed to update application");
            ApplicationUseCaseError::DatabaseError("Failed to update application".to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(ApplicationUseCaseError::ApplicationNotFound())?;
        }

        Ok(UpdateApplicationDtoResponse(UpdateApplicationUseCaseResponse()))
    }

    #[instrument(skip(self), name = "application_repository_delete")]
    async fn delete(
        &self,
        req: DeleteApplicationDtoRequest,
    ) -> Result<DeleteApplicationDtoResponse, ApplicationUseCaseError> {
        let result = sqlx::query!(
            "
            UPDATE applications
            SET deleted_at = ?
            WHERE id = ?
                AND deleted_at IS NULL",
            Some(UtcDateTime::now().value()),
            req.0.id.to_string()
        )
        .execute(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to delete application");
            ApplicationUseCaseError::DatabaseError("Failed to delete application".to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(ApplicationUseCaseError::ApplicationNotFound())?;
        }

        Ok(DeleteApplicationDtoResponse(DeleteApplicationUseCaseResponse()))
    }

    #[instrument(skip(self), name = "application_repository_count")]
    async fn count_applications(
        &self,
        req: CountApplicationsDtoRequest,
    ) -> Result<CountApplicationsDtoResponse, ApplicationUseCaseError> {
        let mut query = String::from("SELECT COUNT(*) AS total FROM applications");
        query.push_str(match req.deleted {
            true => " WHERE deleted_at IS NOT NULL",
            false => " WHERE deleted_at IS NULL",
        });

        let result = sqlx::query(&query)
            .fetch_one(self.db.pool.clone().as_ref())
            .await
            .map_err(|err| {
                error!(error = %err, "Failed to count applications");
                ApplicationUseCaseError::DatabaseError("Failed to count applications".to_string())
            })?;

        Ok(CountApplicationsDtoResponse(result.try_get("total")?))
    }

    #[instrument(skip(self), name = "application_repository_restore")]
    async fn restore(
        &self,
        req: RestoreApplicationDtoRequest,
    ) -> Result<RestoreApplicationDtoResponse, ApplicationUseCaseError> {
        let result = sqlx::query!(
            "
            UPDATE applications
            SET deleted_at = NULL
            WHERE id = ?
                AND deleted_at IS NOT NULL",
            req.0.id.to_string()
        )
        .execute(self.db.pool.clone().as_ref())
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to restore application");
            ApplicationUseCaseError::DatabaseError("Failed to restore application".to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(ApplicationUseCaseError::ApplicationNotFound())?;
        }

        Ok(RestoreApplicationDtoResponse(RestoreApplicationUseCaseResponse()))
    }
}
