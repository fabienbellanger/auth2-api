//! Scope use cases

pub mod create_scope;

use crate::domain::entities::application::ApplicationId;
use crate::domain::entities::scope::ScopeId;
use crate::domain::repositories::scope::ScopeRepository;
use crate::domain::use_cases::scope::create_scope::CreateScopeUseCase;
use crate::domain::value_objects::datetime::UtcDateTime;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ScopeUseCases<S: ScopeRepository> {
    pub create_scope: CreateScopeUseCase<S>,
}

impl<S: ScopeRepository> ScopeUseCases<S> {
    /// Create a new scope use cases
    pub fn new(scope_repository: S) -> Self {
        Self {
            create_scope: CreateScopeUseCase::new(scope_repository.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ScopeUseCaseError {
    #[error("Scope not found")]
    ScopeNotFound(),

    #[error("Invalid scope id: {0}")]
    InvalidId(String),

    #[error("Invalid scope application id: {0}")]
    InvalidApplicationId(String),

    #[error("Model conversion error")]
    FromModelError(),

    #[error("Scope ID already exists")]
    IdAlreadyExists(),

    #[error("{0}")]
    DatabaseError(String),
}

/// Scope use case generic response
#[derive(Debug, Clone)]
pub struct ScopeUseCaseResponse {
    pub id: ScopeId,
    pub application_id: ApplicationId,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
    pub deleted_at: Option<UtcDateTime>,
}
