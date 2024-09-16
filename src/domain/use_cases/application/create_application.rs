//! Create an application use case

use crate::domain::repositories::application::dto::CreateApplicationDtoRequest;
use crate::domain::repositories::application::ApplicationRepository;
use crate::domain::use_cases::application::{ApplicationUseCaseError, ApplicationUseCaseResponse};

#[derive(Debug, Clone)]
pub struct CreateApplicationUseCaseRequest {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct CreateApplicationUseCase<A: ApplicationRepository> {
    application_repository: A,
}

impl<A: ApplicationRepository> CreateApplicationUseCase<A> {
    /// Create a new use case
    pub fn new(application_repository: A) -> Self {
        Self { application_repository }
    }

    /// Create a new application
    #[instrument(skip(self), name = "create_application_use_case")]
    pub async fn call(
        &self,
        request: CreateApplicationUseCaseRequest,
    ) -> Result<ApplicationUseCaseResponse, ApplicationUseCaseError> {
        let application = self
            .application_repository
            .create(CreateApplicationDtoRequest(request))
            .await?;

        Ok(application.0)
    }
}
