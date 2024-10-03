//! Restore deleted scope use case

use crate::domain::entities::scope::ScopeId;
use crate::domain::repositories::scope::dto::RestoreScopeDtoRequest;
use crate::domain::repositories::scope::ScopeRepository;
use crate::domain::use_cases::scope::ScopeUseCaseError;

#[derive(Debug, Clone)]
pub struct RestoreScopeUseCaseRequest {
    pub id: ScopeId,
}

#[derive(Debug, Clone)]
pub struct RestoreScopeUseCaseResponse();

#[derive(Debug, Clone)]
pub struct RestoreScopeUseCase<A: ScopeRepository> {
    scope_repository: A,
}

impl<A: ScopeRepository> RestoreScopeUseCase<A> {
    /// Create a new use case
    pub fn new(scope_repository: A) -> Self {
        Self { scope_repository }
    }

    /// Restore an scope
    #[instrument(skip(self), name = "restore_scope_use_case")]
    pub async fn call(
        &self,
        request: RestoreScopeUseCaseRequest,
    ) -> Result<RestoreScopeUseCaseResponse, ScopeUseCaseError> {
        let scope = self.scope_repository.restore(RestoreScopeDtoRequest(request)).await?;

        Ok(scope.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::mock::scope::{ScopeRepositoryMock, INVALID_SCOPE_ID, VALID_SCOPE_ID};

    #[tokio::test]
    async fn test_restore_scope_use_case() {
        let scope_repository = ScopeRepositoryMock {};
        let use_case = RestoreScopeUseCase::new(scope_repository);

        let request = RestoreScopeUseCaseRequest {
            id: VALID_SCOPE_ID.to_string(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_restore_scope_use_case_with_error() {
        let scope_repository = ScopeRepositoryMock {};
        let use_case = RestoreScopeUseCase::new(scope_repository);

        let request = RestoreScopeUseCaseRequest {
            id: INVALID_SCOPE_ID.to_string(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_err());
        if let Err(err) = response {
            assert_eq!(
                err,
                ScopeUseCaseError::DatabaseError("Failed to restore scope".to_string())
            );
        }
    }
}
