//! Delete application use case

use crate::domain::entities::application::ApplicationId;
use crate::domain::repositories::application::dto::DeleteApplicationDtoRequest;
use crate::domain::repositories::application::ApplicationRepository;
use crate::domain::use_cases::application::ApplicationUseCaseError;

#[derive(Debug, Clone)]
pub struct DeleteApplicationUseCaseRequest {
    pub id: ApplicationId,
}

#[derive(Debug, Clone)]
pub struct DeleteApplicationUseCaseResponse();

#[derive(Debug, Clone)]
pub struct DeleteApplicationUseCase<A: ApplicationRepository> {
    application_repository: A,
}

impl<A: ApplicationRepository> DeleteApplicationUseCase<A> {
    /// Create a new use case
    pub fn new(application_repository: A) -> Self {
        Self { application_repository }
    }

    /// Delete an application
    #[instrument(skip(self), name = "delete_application_use_case")]
    pub async fn call(
        &self,
        request: DeleteApplicationUseCaseRequest,
    ) -> Result<DeleteApplicationUseCaseResponse, ApplicationUseCaseError> {
        let application = self
            .application_repository
            .delete(DeleteApplicationDtoRequest(request))
            .await?;

        Ok(application.0)
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
