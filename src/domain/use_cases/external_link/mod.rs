//! External link use cases

pub mod create_external_link;
pub mod delete_external_link;
pub mod get_external_link;
pub mod get_external_links;
pub mod restore_external_link;
pub mod update_external_link;

use crate::domain::{
    entities::external_link::ExternalLinkId, repositories::external_link::ExternalLinkRepository,
    value_objects::datetime::UtcDateTime,
};
use create_external_link::CreateExternalLinkUseCase;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ExternalLinkUseCases<L: ExternalLinkRepository> {
    pub create_external_link: create_external_link::CreateExternalLinkUseCase<L>,
    pub get_external_link: get_external_link::GetExternalLinkByIdUseCase<L>,
    pub get_external_links: get_external_links::GetExternalLinksUseCase<L>,
    pub update_external_link: update_external_link::UpdateExternalLinkUseCase<L>,
    pub delete_external_link: delete_external_link::DeleteExternalLinkUseCase<L>,
    pub restore_external_link: restore_external_link::RestoreExternalLinkUseCase<L>,
}

impl<L: ExternalLinkRepository> ExternalLinkUseCases<L> {
    /// Create a new external link use cases
    pub fn new(external_link_repository: L) -> Self {
        Self {
            create_external_link: CreateExternalLinkUseCase::new(external_link_repository.clone()),
            get_external_link: get_external_link::GetExternalLinkByIdUseCase::new(external_link_repository.clone()),
            get_external_links: get_external_links::GetExternalLinksUseCase::new(external_link_repository.clone()),
            update_external_link: update_external_link::UpdateExternalLinkUseCase::new(
                external_link_repository.clone(),
            ),
            delete_external_link: delete_external_link::DeleteExternalLinkUseCase::new(
                external_link_repository.clone(),
            ),
            restore_external_link: restore_external_link::RestoreExternalLinkUseCase::new(
                external_link_repository.clone(),
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ExternalLinkUseCaseError {
    #[error("External link not found")]
    ExternalLinkNotFound(),

    #[error("Invalid external link id")]
    InvalidId(),

    #[error("Model conversion error")]
    FromModelError(),

    #[error("{0}")]
    DatabaseError(String),
}

/// External link use case generic response
#[derive(Debug, Clone, PartialEq)]
pub struct ExternalLinkUseCaseResponse {
    pub id: ExternalLinkId,
    pub name: String,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
    pub deleted_at: Option<UtcDateTime>,
}
