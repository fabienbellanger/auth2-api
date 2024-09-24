//! Restore deleted application use case

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
    // TODO
}
