//! Get all scopes use case

use crate::domain::entities::application::ApplicationId;
use crate::domain::repositories::scope::ScopeRepository;
use crate::domain::repositories::scope::dto::{CountScopesDtoRequest, GetScopesDtoRequest};
use crate::domain::use_cases::scope::{ScopeUseCaseError, ScopeUseCaseResponse};
use crate::domain::value_objects::pagination::Pagination;
use crate::domain::value_objects::query_sort::QuerySorts;
use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct GetScopesUseCaseRequest {
    pub pagination: Pagination,
    pub sorts: Option<QuerySorts>,
    pub deleted: bool,
    pub application_id: Option<ApplicationId>,
}

#[derive(Debug, Clone)]
pub struct GetScopesUseCaseResponse {
    pub scopes: Vec<ScopeUseCaseResponse>,
    pub total: i64,
}

#[derive(Debug, Clone)]
pub struct GetScopesUseCase<S: ScopeRepository> {
    scope_repository: S,
}

impl<S: ScopeRepository> GetScopesUseCase<S> {
    /// Create a new use case
    pub fn new(scope_repository: S) -> Self {
        Self { scope_repository }
    }

    /// Get scopes
    #[instrument(skip(self), name = "get_scopes_use_case")]
    pub async fn call(&self, request: GetScopesUseCaseRequest) -> Result<GetScopesUseCaseResponse, ScopeUseCaseError> {
        let total = self
            .scope_repository
            .count_scopes(CountScopesDtoRequest {
                deleted: request.deleted,
                application_id: request.application_id.clone().map(|id| id.to_string()),
            })
            .await?
            .0;
        let scopes = self.scope_repository.get_scopes(GetScopesDtoRequest(request)).await?.0;

        Ok(GetScopesUseCaseResponse { scopes, total })
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
