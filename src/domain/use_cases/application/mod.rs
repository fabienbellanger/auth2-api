//! Application use cases

pub mod create_application;
pub mod delete_application;
pub mod get_application;
pub mod get_applications;
pub mod update_application;

use crate::domain::entities::application::ApplicationId;
use crate::domain::repositories::application::ApplicationRepository;
use crate::domain::use_cases::application::create_application::CreateApplicationUseCase;
use crate::domain::use_cases::application::delete_application::DeleteApplicationUseCase;
use crate::domain::use_cases::application::get_application::GetApplicationByIdUseCase;
use crate::domain::use_cases::application::get_applications::GetApplicationsUseCase;
use crate::domain::use_cases::application::update_application::UpdateApplicationUseCase;
use crate::domain::value_objects::datetime::UtcDateTime;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ApplicationUseCases<A: ApplicationRepository> {
    pub create_application: CreateApplicationUseCase<A>,
    pub get_application: GetApplicationByIdUseCase<A>,
    pub get_applications: GetApplicationsUseCase<A>,
    pub update_application: UpdateApplicationUseCase<A>,
    pub delete_application: DeleteApplicationUseCase<A>,
}

impl<A: ApplicationRepository> ApplicationUseCases<A> {
    /// Create a new application use cases
    pub fn new(application_repository: A) -> Self {
        Self {
            create_application: CreateApplicationUseCase::new(application_repository.clone()),
            get_application: GetApplicationByIdUseCase::new(application_repository.clone()),
            get_applications: GetApplicationsUseCase::new(application_repository.clone()),
            update_application: UpdateApplicationUseCase::new(application_repository.clone()),
            delete_application: DeleteApplicationUseCase::new(application_repository.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ApplicationUseCaseError {
    #[error("Application not found")]
    ApplicationNotFound(),

    #[error("Invalid application id")]
    InvalidId(),

    #[error("Model conversion error")]
    FromModelError(),

    #[error("Invalid name: {0}")]
    InvalidName(String),

    #[error("{0}")]
    DatabaseError(String),
}

/// Application use case generic response
#[derive(Debug, Clone)]
pub struct ApplicationUseCaseResponse {
    pub id: ApplicationId,
    pub name: String,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
    pub deleted_at: Option<UtcDateTime>,
}
