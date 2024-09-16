//! Application repository

use crate::domain::repositories::application::dto::{CreateApplicationDtoRequest, CreateApplicationDtoResponse};
use crate::domain::use_cases::application::ApplicationUseCaseError;
use async_trait::async_trait;

pub mod dto;

#[async_trait]
pub trait ApplicationRepository: Clone {
    /// Create application
    async fn create(
        &self,
        req: CreateApplicationDtoRequest,
    ) -> Result<CreateApplicationDtoResponse, ApplicationUseCaseError>;

    // /// Get application by ID
    // async fn get_by_id(&self, request: dto::GetApplicationByIdDtoRequest) -> Result<dto::GetApplicationByIdDtoResponse, dto::GetApplicationByIdDtoError>;
    //
    // /// Get all applications
    // async fn get_all(&self) -> Result<dto::GetAllApplicationsDtoResponse, dto::GetAllApplicationsDtoError>;
    //
    // /// Update application
    // async fn update(&self, request: dto::UpdateApplicationDtoRequest) -> Result<dto::UpdateApplicationDtoResponse, dto::UpdateApplicationDtoError>;
    //
    // /// Delete application
    // async fn delete(&self, request: dto::DeleteApplicationDtoRequest) -> Result<dto::DeleteApplicationDtoResponse, dto::DeleteApplicationDtoError>;
}
