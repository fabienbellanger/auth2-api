//! Scope repository

use crate::domain::repositories::scope::dto::{
    CountScopesDtoRequest, CountScopesDtoResponse, CreateScopeDtoRequest, CreateScopeDtoResponse,
    DeleteScopeDtoRequest, DeleteScopeDtoResponse, GetScopesDtoRequest, GetScopesDtoResponse, RestoreScopeDtoRequest,
    RestoreScopeDtoResponse,
};
use crate::domain::use_cases::scope::ScopeUseCaseError;
use async_trait::async_trait;

pub mod dto;

#[async_trait]
pub trait ScopeRepository: Clone {
    /// Create scope
    async fn create(&self, req: CreateScopeDtoRequest) -> Result<CreateScopeDtoResponse, ScopeUseCaseError>;

    /// Get all scopes
    async fn get_scopes(&self, req: GetScopesDtoRequest) -> Result<GetScopesDtoResponse, ScopeUseCaseError>;

    /// Count all scopes
    async fn count_scopes(&self, req: CountScopesDtoRequest) -> Result<CountScopesDtoResponse, ScopeUseCaseError>;

    /// Delete scope
    async fn delete(&self, req: DeleteScopeDtoRequest) -> Result<DeleteScopeDtoResponse, ScopeUseCaseError>;

    /// Restore deleted scope
    async fn restore(&self, req: RestoreScopeDtoRequest) -> Result<RestoreScopeDtoResponse, ScopeUseCaseError>;
}
