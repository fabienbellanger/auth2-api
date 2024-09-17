//! Mock of the application repository

use crate::domain::repositories::application::dto::{
    CountApplicationsDtoRequest, CountApplicationsDtoResponse, CreateApplicationDtoRequest,
    CreateApplicationDtoResponse, GetApplicationByIdDtoRequest, GetApplicationByIdDtoResponse,
    GetApplicationsDtoRequest, GetApplicationsDtoResponse,
};
use crate::domain::repositories::application::{dto, ApplicationRepository};
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
            }))
        } else {
            Err(ApplicationUseCaseError::ApplicationNotFound)
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
        _req: dto::UpdateApplicationDtoRequest,
    ) -> Result<dto::UpdateApplicationDtoResponse, ApplicationUseCaseError> {
        todo!()
    }

    /// Delete application
    async fn delete(
        &self,
        _req: dto::DeleteApplicationDtoRequest,
    ) -> Result<dto::DeleteApplicationDtoResponse, ApplicationUseCaseError> {
        todo!()
    }

    /// Count all applications
    async fn count_applications(
        &self,
        _req: CountApplicationsDtoRequest,
    ) -> Result<CountApplicationsDtoResponse, ApplicationUseCaseError> {
        todo!()
    }
}
