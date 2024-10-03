//! Restore restored application use case

use crate::domain::entities::application::ApplicationId;
use crate::domain::repositories::application::dto::RestoreApplicationDtoRequest;
use crate::domain::repositories::application::ApplicationRepository;
use crate::domain::use_cases::application::ApplicationUseCaseError;

#[derive(Debug, Clone)]
pub struct RestoreApplicationUseCaseRequest {
    pub id: ApplicationId,
}

#[derive(Debug, Clone)]
pub struct RestoreApplicationUseCaseResponse();

#[derive(Debug, Clone)]
pub struct RestoreApplicationUseCase<A: ApplicationRepository> {
    application_repository: A,
}

impl<A: ApplicationRepository> RestoreApplicationUseCase<A> {
    /// Create a new use case
    pub fn new(application_repository: A) -> Self {
        Self { application_repository }
    }

    /// Restore an application
    #[instrument(skip(self), name = "restore_application_use_case")]
    pub async fn call(
        &self,
        request: RestoreApplicationUseCaseRequest,
    ) -> Result<RestoreApplicationUseCaseResponse, ApplicationUseCaseError> {
        let application = self
            .application_repository
            .restore(RestoreApplicationDtoRequest(request))
            .await?;

        Ok(application.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        tests::mock::application::{ApplicationRepositoryMock, INVALID_APPLICATION_ID, VALID_APPLICATION_ID},
        value_objects::id::Id,
    };
    use std::str::FromStr;

    #[tokio::test]
    async fn test_restore_application_use_case() {
        let application_repository = ApplicationRepositoryMock {};
        let use_case = RestoreApplicationUseCase::new(application_repository);

        let request = RestoreApplicationUseCaseRequest {
            id: Id::from_str(VALID_APPLICATION_ID).unwrap(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_restore_application_use_case_with_error() {
        let application_repository = ApplicationRepositoryMock {};
        let use_case = RestoreApplicationUseCase::new(application_repository);

        let request = RestoreApplicationUseCaseRequest {
            id: Id::from_str(INVALID_APPLICATION_ID).unwrap(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_err());
        if let Err(err) = response {
            assert_eq!(
                err,
                ApplicationUseCaseError::DatabaseError("Failed to restore application".to_string())
            );
        }
    }
}
