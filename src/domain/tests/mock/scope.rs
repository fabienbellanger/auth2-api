//! Mock of the scope repository

use crate::domain::repositories::scope::dto::{
    CountScopesDtoRequest, CountScopesDtoResponse, CreateScopeDtoRequest, CreateScopeDtoResponse,
    DeleteScopeDtoRequest, DeleteScopeDtoResponse, GetScopesDtoRequest, GetScopesDtoResponse, RestoreScopeDtoRequest,
    RestoreScopeDtoResponse,
};
use crate::domain::repositories::scope::ScopeRepository;
use crate::domain::tests::mock::application::VALID_APPLICATION_ID;
use crate::domain::use_cases::scope::{ScopeUseCaseError, ScopeUseCaseResponse};
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::id::Id;
use async_trait::async_trait;
use std::str::FromStr;

pub const VALID_SCOPE_ID: &str = "user:read";
pub const INVALID_SCOPE_ID: &str = "test";

/// Scope repository mock
#[derive(Debug, Clone)]
pub struct ScopeRepositoryMock {}

#[async_trait]
impl ScopeRepository for ScopeRepositoryMock {
    /// Create scope
    async fn create(&self, req: CreateScopeDtoRequest) -> Result<CreateScopeDtoResponse, ScopeUseCaseError> {
        if req.0.id == INVALID_SCOPE_ID {
            Err(ScopeUseCaseError::DatabaseError("Scope ID already exists".to_string()))
        } else {
            let now = UtcDateTime::now();
            Ok(CreateScopeDtoResponse(ScopeUseCaseResponse {
                id: VALID_SCOPE_ID.to_string(),
                application_id: Id::from_str(VALID_APPLICATION_ID).unwrap(),
                created_at: now.clone(),
                updated_at: now,
                deleted_at: None,
            }))
        }
    }

    /// Get all scopes
    async fn get_scopes(&self, _req: GetScopesDtoRequest) -> Result<GetScopesDtoResponse, ScopeUseCaseError> {
        todo!()
    }

    /// Count all scopes
    async fn count_scopes(&self, _req: CountScopesDtoRequest) -> Result<CountScopesDtoResponse, ScopeUseCaseError> {
        todo!()
    }

    /// Delete scope
    async fn delete(&self, _req: DeleteScopeDtoRequest) -> Result<DeleteScopeDtoResponse, ScopeUseCaseError> {
        todo!()
    }

    /// Restore deleted scope
    async fn restore(&self, _req: RestoreScopeDtoRequest) -> Result<RestoreScopeDtoResponse, ScopeUseCaseError> {
        todo!()
    }
}
