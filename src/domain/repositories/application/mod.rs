//! Application repository

use crate::domain::repositories::application::dto::{
    CountApplicationsDtoRequest, CountApplicationsDtoResponse, CreateApplicationDtoRequest,
    CreateApplicationDtoResponse, DeleteApplicationDtoRequest, DeleteApplicationDtoResponse,
    GetApplicationByIdDtoRequest, GetApplicationByIdDtoResponse, GetApplicationsDtoRequest, GetApplicationsDtoResponse,
    RestoreApplicationDtoRequest, RestoreApplicationDtoResponse, UpdateApplicationDtoRequest,
    UpdateApplicationDtoResponse,
};
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

    /// Get application by ID
    async fn get_by_id(
        &self,
        req: GetApplicationByIdDtoRequest,
    ) -> Result<GetApplicationByIdDtoResponse, ApplicationUseCaseError>;

    /// Get all applications
    async fn get_applications(
        &self,
        req: GetApplicationsDtoRequest,
    ) -> Result<GetApplicationsDtoResponse, ApplicationUseCaseError>;

    /// Update application
    async fn update(
        &self,
        req: UpdateApplicationDtoRequest,
    ) -> Result<UpdateApplicationDtoResponse, ApplicationUseCaseError>;

    /// Delete application
    async fn delete(
        &self,
        req: DeleteApplicationDtoRequest,
    ) -> Result<DeleteApplicationDtoResponse, ApplicationUseCaseError>;

    /// Count all applications
    async fn count_applications(
        &self,
        req: CountApplicationsDtoRequest,
    ) -> Result<CountApplicationsDtoResponse, ApplicationUseCaseError>;

    /// Restore deleted application
    async fn restore(
        &self,
        req: RestoreApplicationDtoRequest,
    ) -> Result<RestoreApplicationDtoResponse, ApplicationUseCaseError>;
}
