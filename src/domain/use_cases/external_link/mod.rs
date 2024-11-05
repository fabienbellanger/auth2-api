//! External link use cases

pub mod create_external_link;

use crate::domain::{
    entities::external_link::ExternalLinkId, repositories::external_link::ExternalLinkRepository,
    value_objects::datetime::UtcDateTime,
};
use create_external_link::CreateExternalLinkUseCase;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ExternalLinkUseCases<L: ExternalLinkRepository> {
    pub create_external_link: create_external_link::CreateExternalLinkUseCase<L>,
}

impl<L: ExternalLinkRepository> ExternalLinkUseCases<L> {
    /// Create a new external link use cases
    pub fn new(external_link_repository: L) -> Self {
        Self {
            create_external_link: CreateExternalLinkUseCase::new(external_link_repository.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ExternalLinkUseCaseError {
    #[error("Model conversion error")]
    FromModelError(),

    #[error("{0}")]
    DatabaseError(String),
}

/// External link use case generic response
#[derive(Debug, Clone)]
pub struct ExternalLinkUseCaseResponse {
    pub id: ExternalLinkId,
    pub name: String,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
    pub deleted_at: Option<UtcDateTime>,
}
