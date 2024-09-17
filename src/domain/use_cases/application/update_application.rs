//! Update application use case

use crate::domain::entities::application::ApplicationId;
use crate::domain::repositories::application::dto::UpdateApplicationDtoRequest;
use crate::domain::repositories::application::ApplicationRepository;
use crate::domain::use_cases::application::ApplicationUseCaseError;

#[derive(Debug, Clone)]
pub struct UpdateApplicationUseCaseRequest {
    pub id: ApplicationId,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct UpdateApplicationUseCaseResponse();

#[derive(Debug, Clone)]
pub struct UpdateApplicationUseCase<A: ApplicationRepository> {
    application_repository: A,
}

impl<A: ApplicationRepository> UpdateApplicationUseCase<A> {
    /// Create a new use case
    pub fn new(application_repository: A) -> Self {
        Self { application_repository }
    }

    /// Update an application
    #[instrument(skip(self), name = "update_application_use_case")]
    pub async fn call(
        &self,
        request: UpdateApplicationUseCaseRequest,
    ) -> Result<UpdateApplicationUseCaseResponse, ApplicationUseCaseError> {
        let application = self
            .application_repository
            .update(UpdateApplicationDtoRequest(request))
            .await?;

        Ok(application.0)
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
