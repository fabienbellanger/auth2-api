//! Get all applications with pagination use case

use crate::domain::repositories::application::ApplicationRepository;
use crate::domain::repositories::application::dto::{CountApplicationsDtoRequest, GetApplicationsDtoRequest};
use crate::domain::use_cases::application::{ApplicationUseCaseError, ApplicationUseCaseResponse};
use crate::domain::value_objects::pagination::Pagination;
use crate::domain::value_objects::query_sort::QuerySorts;

#[derive(Debug, Clone)]
pub struct GetApplicationsUseCaseRequest {
    pub pagination: Pagination,
    pub sorts: Option<QuerySorts>,
    // pub filters: Option<Filters>,
    pub deleted: bool,
}

#[derive(Debug, Clone)]
pub struct GetApplicationsUseCaseResponse {
    pub applications: Vec<ApplicationUseCaseResponse>,
    pub total: i64,
}

#[derive(Debug, Clone)]
pub struct GetApplicationsUseCase<A: ApplicationRepository> {
    application_repository: A,
}

impl<A: ApplicationRepository> GetApplicationsUseCase<A> {
    /// Create a new use case
    pub fn new(application_repository: A) -> Self {
        Self { application_repository }
    }

    /// Get all applications
    #[instrument(skip(self), name = "get_applications_use_case")]
    pub async fn call(
        &self,
        request: GetApplicationsUseCaseRequest,
    ) -> Result<GetApplicationsUseCaseResponse, ApplicationUseCaseError> {
        let total = self
            .application_repository
            .count_applications(CountApplicationsDtoRequest {
                deleted: request.deleted,
            })
            .await?
            .0;
        let applications = self
            .application_repository
            .get_applications(GetApplicationsDtoRequest(request))
            .await?
            .0;

        Ok(GetApplicationsUseCaseResponse { applications, total })
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
