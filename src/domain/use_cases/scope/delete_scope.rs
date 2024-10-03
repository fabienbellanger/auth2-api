//! Delete scope use case

use crate::domain::entities::scope::ScopeId;
use crate::domain::repositories::scope::dto::DeleteScopeDtoRequest;
use crate::domain::repositories::scope::ScopeRepository;
use crate::domain::use_cases::scope::ScopeUseCaseError;

#[derive(Debug, Clone)]
pub struct DeleteScopeUseCaseRequest {
    pub id: ScopeId,
}

#[derive(Debug, Clone)]
pub struct DeleteScopeUseCaseResponse();

#[derive(Debug, Clone)]
pub struct DeleteScopeUseCase<A: ScopeRepository> {
    scope_repository: A,
}

impl<A: ScopeRepository> DeleteScopeUseCase<A> {
    /// Create a new use case
    pub fn new(scope_repository: A) -> Self {
        Self { scope_repository }
    }

    /// Delete an scope
    #[instrument(skip(self), name = "delete_scope_use_case")]
    pub async fn call(
        &self,
        request: DeleteScopeUseCaseRequest,
    ) -> Result<DeleteScopeUseCaseResponse, ScopeUseCaseError> {
        let scope = self.scope_repository.delete(DeleteScopeDtoRequest(request)).await?;

        Ok(scope.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::mock::scope::{ScopeRepositoryMock, INVALID_SCOPE_ID, VALID_SCOPE_ID};

    #[tokio::test]
    async fn test_delete_scope_use_case() {
        let scope_repository = ScopeRepositoryMock {};
        let use_case = DeleteScopeUseCase::new(scope_repository);

        let request = DeleteScopeUseCaseRequest {
            id: VALID_SCOPE_ID.to_string(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_delete_scope_use_case_with_error() {
        let scope_repository = ScopeRepositoryMock {};
        let use_case = DeleteScopeUseCase::new(scope_repository);

        let request = DeleteScopeUseCaseRequest {
            id: INVALID_SCOPE_ID.to_string(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_err());
        if let Err(err) = response {
            assert_eq!(
                err,
                ScopeUseCaseError::DatabaseError("Failed to delete scope".to_string())
            );
        }
    }
}
