//! Get an application by its ID use case

use crate::domain::entities::application::ApplicationId;
use crate::domain::repositories::application::ApplicationRepository;
use crate::domain::repositories::application::dto::GetApplicationByIdDtoRequest;
use crate::domain::use_cases::application::{ApplicationUseCaseError, ApplicationUseCaseResponse};

#[derive(Debug, Clone)]
pub struct GetApplicationByIdUseCaseRequest {
    pub id: ApplicationId,
}

#[derive(Debug, Clone)]
pub struct GetApplicationByIdUseCase<A: ApplicationRepository> {
    application_repository: A,
}

impl<A: ApplicationRepository> GetApplicationByIdUseCase<A> {
    /// Create a new use case
    pub fn new(application_repository: A) -> Self {
        Self { application_repository }
    }

    /// Get an application by its ID
    #[instrument(skip(self), name = "get_application_use_case")]
    pub async fn call(
        &self,
        request: GetApplicationByIdUseCaseRequest,
    ) -> Result<ApplicationUseCaseResponse, ApplicationUseCaseError> {
        let application = self
            .application_repository
            .get_by_id(GetApplicationByIdDtoRequest(request))
            .await?;

        Ok(application.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::mock::application::{
        ApplicationRepositoryMock, INVALID_APPLICATION_ID, VALID_APPLICATION_ID,
    };
    use crate::domain::value_objects::id::Id;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_get_application() {
        let application_repository = ApplicationRepositoryMock {};
        let use_case = GetApplicationByIdUseCase::new(application_repository);

        let request = GetApplicationByIdUseCaseRequest {
            id: Id::from_str(VALID_APPLICATION_ID).unwrap(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_get_application_error() {
        let application_repository = ApplicationRepositoryMock {};
        let use_case = GetApplicationByIdUseCase::new(application_repository);

        let request = GetApplicationByIdUseCaseRequest {
            id: Id::from_str(INVALID_APPLICATION_ID).unwrap(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_err());
    }
}
