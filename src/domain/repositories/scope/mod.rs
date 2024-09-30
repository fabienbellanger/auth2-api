//! Scope repository

use crate::domain::repositories::scope::dto::{CreateScopeDtoRequest, CreateScopeDtoResponse};
use crate::domain::use_cases::scope::ScopeUseCaseError;
use async_trait::async_trait;

pub mod dto;

#[async_trait]
pub trait ScopeRepository: Clone {
    /// Create scope
    async fn create(&self, req: CreateScopeDtoRequest) -> Result<CreateScopeDtoResponse, ScopeUseCaseError>;
}
