//! Scope use cases

pub mod create_scope;
pub mod delete_scope;
pub mod get_scopes;
pub mod restore_scope;

use crate::domain::entities::application::ApplicationId;
use crate::domain::entities::scope::ScopeId;
use crate::domain::repositories::scope::ScopeRepository;
use crate::domain::use_cases::scope::create_scope::CreateScopeUseCase;
use crate::domain::use_cases::scope::delete_scope::DeleteScopeUseCase;
use crate::domain::use_cases::scope::get_scopes::GetScopesUseCase;
use crate::domain::use_cases::scope::restore_scope::RestoreScopeUseCase;
use crate::domain::value_objects::datetime::UtcDateTime;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ScopeUseCases<S: ScopeRepository> {
    pub create_scope: CreateScopeUseCase<S>,
    pub get_scopes: GetScopesUseCase<S>,
    pub delete_scope: DeleteScopeUseCase<S>,
    pub restore_scope: RestoreScopeUseCase<S>,
}

impl<S: ScopeRepository> ScopeUseCases<S> {
    /// Create a new scope use cases
    pub fn new(scope_repository: S) -> Self {
        Self {
            create_scope: CreateScopeUseCase::new(scope_repository.clone()),
            get_scopes: GetScopesUseCase::new(scope_repository.clone()),
            delete_scope: DeleteScopeUseCase::new(scope_repository.clone()),
            restore_scope: RestoreScopeUseCase::new(scope_repository.clone()),
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
