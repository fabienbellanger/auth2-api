//! Create scope use case

use crate::domain::entities::application::ApplicationId;
use crate::domain::entities::scope::ScopeId;
use crate::domain::repositories::scope::dto::CreateScopeDtoRequest;
use crate::domain::repositories::scope::ScopeRepository;
use crate::domain::use_cases::scope::{ScopeUseCaseError, ScopeUseCaseResponse};
use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct CreateScopeUseCaseRequest {
    #[validate(length(min = 4))]
    pub id: ScopeId,
    pub application_id: ApplicationId,
}

#[derive(Debug, Clone)]
pub struct CreateScopeUseCase<S: ScopeRepository> {
    scope_repository: S,
}

impl<S: ScopeRepository> CreateScopeUseCase<S> {
    /// Create a new use case
    pub fn new(scope_repository: S) -> Self {
        Self { scope_repository }
    }

    /// Create a new scope
    #[instrument(skip(self), name = "create_scope_use_case")]
    pub async fn call(&self, request: CreateScopeUseCaseRequest) -> Result<ScopeUseCaseResponse, ScopeUseCaseError> {
        if let Err(err) = request.validate() {
            return Err(ScopeUseCaseError::InvalidId(err.to_string()));
        }

        let scope = self.scope_repository.create(CreateScopeDtoRequest(request)).await?;

        Ok(scope.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::mock::application::VALID_APPLICATION_ID;
    use crate::domain::tests::mock::scope::{ScopeRepositoryMock, INVALID_SCOPE_ID, VALID_SCOPE_ID};
    use crate::domain::value_objects::id::Id;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_create_scope() {
        let repository = ScopeRepositoryMock {};
        let create_scope = CreateScopeUseCase::new(repository);

        let result = create_scope
            .call(CreateScopeUseCaseRequest {
                id: VALID_SCOPE_ID.to_string(),
                application_id: Id::from_str(VALID_APPLICATION_ID).unwrap(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_scope_invalid_id() {
        let repository = ScopeRepositoryMock {};
        let create_scope = CreateScopeUseCase::new(repository);

        let result = create_scope
            .call(CreateScopeUseCaseRequest {
                id: "one".to_string(),
                application_id: Id::from_str(VALID_APPLICATION_ID).unwrap(),
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_scope_database_error() {
        let repository = ScopeRepositoryMock {};
        let create_scope = CreateScopeUseCase::new(repository);

        let result = create_scope
            .call(CreateScopeUseCaseRequest {
                id: INVALID_SCOPE_ID.to_string(),
                application_id: Id::from_str(VALID_APPLICATION_ID).unwrap(),
            })
            .await;

        assert!(result.is_err());
    }
}
