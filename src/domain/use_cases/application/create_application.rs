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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::mock::application::{
        ApplicationRepositoryMock, INVALID_APPLICATION_NAME, VALID_APPLICATION_NAME,
    };

    #[tokio::test]
    async fn test_create_application() {
        let application_repository = ApplicationRepositoryMock {};
        let use_case = CreateApplicationUseCase::new(application_repository);

        let request = CreateApplicationUseCaseRequest {
            name: VALID_APPLICATION_NAME.to_string(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_create_application_error() {
        let application_repository = ApplicationRepositoryMock {};
        let use_case = CreateApplicationUseCase::new(application_repository);

        let request = CreateApplicationUseCaseRequest {
            name: INVALID_APPLICATION_NAME.to_string(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_err());
    }
}
