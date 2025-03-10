//! Mock of the application repository

use crate::domain::repositories::application::ApplicationRepository;
use crate::domain::repositories::application::dto::{
    CountApplicationsDtoRequest, CountApplicationsDtoResponse, CreateApplicationDtoRequest,
    CreateApplicationDtoResponse, DeleteApplicationDtoRequest, DeleteApplicationDtoResponse,
    GetApplicationByIdDtoRequest, GetApplicationByIdDtoResponse, GetApplicationsDtoRequest, GetApplicationsDtoResponse,
    RestoreApplicationDtoRequest, RestoreApplicationDtoResponse, UpdateApplicationDtoRequest,
    UpdateApplicationDtoResponse,
};
use crate::domain::use_cases::application::delete_application::DeleteApplicationUseCaseResponse;
use crate::domain::use_cases::application::restore_application::RestoreApplicationUseCaseResponse;
use crate::domain::use_cases::application::{ApplicationUseCaseError, ApplicationUseCaseResponse};
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::id::Id;
use async_trait::async_trait;
use std::str::FromStr;

pub const VALID_APPLICATION_ID: &str = "ffaa2c9c-872f-4e62-8302-d4586096cd13";
pub const INVALID_APPLICATION_ID: &str = "b4dc6179-e538-449b-accd-a8a1f58631af";
pub const VALID_APPLICATION_NAME: &str = "Test application";
pub const INVALID_APPLICATION_NAME: &str = "Test invalid application";

/// Application repository mock
#[derive(Debug, Clone)]
pub struct ApplicationRepositoryMock {}

#[async_trait]
impl ApplicationRepository for ApplicationRepositoryMock {
    /// Create application
    async fn create(
        &self,
        req: CreateApplicationDtoRequest,
    ) -> Result<CreateApplicationDtoResponse, ApplicationUseCaseError> {
        if req.0.name == VALID_APPLICATION_NAME {
            let now = UtcDateTime::now();

            Ok(CreateApplicationDtoResponse(ApplicationUseCaseResponse {
                id: Id::from_str(VALID_APPLICATION_ID).unwrap(),
                name: VALID_APPLICATION_NAME.to_string(),
                created_at: now.clone(),
                updated_at: now,
                deleted_at: None,
            }))
        } else {
            Err(ApplicationUseCaseError::DatabaseError(
                "error during application creation".to_string(),
            ))
        }
    }

    /// Get application by ID
    async fn get_by_id(
        &self,
        req: GetApplicationByIdDtoRequest,
    ) -> Result<GetApplicationByIdDtoResponse, ApplicationUseCaseError> {
        if req.0.id.to_string() == VALID_APPLICATION_ID {
            let now = UtcDateTime::now();

            Ok(GetApplicationByIdDtoResponse(ApplicationUseCaseResponse {
                id: Id::from_str(VALID_APPLICATION_ID).unwrap(),
                name: "Test application".to_string(),
                created_at: now.clone(),
                updated_at: now,
                deleted_at: None,
            }))
        } else {
            Err(ApplicationUseCaseError::ApplicationNotFound())
        }
    }

    /// Get all applications
    async fn get_applications(
        &self,
        _req: GetApplicationsDtoRequest,
    ) -> Result<GetApplicationsDtoResponse, ApplicationUseCaseError> {
        todo!()
    }

    /// Update application
    async fn update(
        &self,
        _req: UpdateApplicationDtoRequest,
    ) -> Result<UpdateApplicationDtoResponse, ApplicationUseCaseError> {
        todo!()
    }

    /// Delete application
    async fn delete(
        &self,
        req: DeleteApplicationDtoRequest,
    ) -> Result<DeleteApplicationDtoResponse, ApplicationUseCaseError> {
        if req.0.id == Id::from_str(VALID_APPLICATION_ID).unwrap() {
            Ok(DeleteApplicationDtoResponse(DeleteApplicationUseCaseResponse()))
        } else {
            Err(ApplicationUseCaseError::DatabaseError(
                "Failed to delete application".to_string(),
            ))
        }
    }

    /// Count all applications
    async fn count_applications(
        &self,
        _req: CountApplicationsDtoRequest,
    ) -> Result<CountApplicationsDtoResponse, ApplicationUseCaseError> {
        todo!()
    }

    /// Restore deleted application
    async fn restore(
        &self,
        req: RestoreApplicationDtoRequest,
    ) -> Result<RestoreApplicationDtoResponse, ApplicationUseCaseError> {
        if req.0.id == Id::from_str(VALID_APPLICATION_ID).unwrap() {
            Ok(RestoreApplicationDtoResponse(RestoreApplicationUseCaseResponse()))
        } else {
            Err(ApplicationUseCaseError::DatabaseError(
                "Failed to restore application".to_string(),
            ))
        }
    }
}
