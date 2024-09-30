//! Mock of the scope repository

use crate::domain::repositories::scope::dto::{CreateScopeDtoRequest, CreateScopeDtoResponse};
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
            Err(ScopeUseCaseError::DatabaseError("".to_string()))
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
}
