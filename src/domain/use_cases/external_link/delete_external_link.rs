//! Delete external link use case

use super::ExternalLinkUseCaseError;
use crate::domain::{
    entities::external_link::ExternalLinkId,
    repositories::external_link::{ExternalLinkRepository, dto::DeleteExternalLinkDtoRequest},
};

#[derive(Debug, Clone)]
pub struct DeleteExternalLinkUseCaseRequest {
    pub id: ExternalLinkId,
}

#[derive(Debug, Clone)]
pub struct DeleteExternalLinkUseCaseResponse();

#[derive(Debug, Clone)]
pub struct DeleteExternalLinkUseCase<L: ExternalLinkRepository> {
    external_link_repository: L,
}

impl<L: ExternalLinkRepository> DeleteExternalLinkUseCase<L> {
    /// Create a new use case
    pub fn new(external_link_repository: L) -> Self {
        Self {
            external_link_repository,
        }
    }

    /// Delete an external link
    #[instrument(skip(self), name = "delete_external_link_use_case")]
    pub async fn call(
        &self,
        request: DeleteExternalLinkUseCaseRequest,
    ) -> Result<DeleteExternalLinkUseCaseResponse, ExternalLinkUseCaseError> {
        let external_link = self
            .external_link_repository
            .delete(DeleteExternalLinkDtoRequest(request))
            .await?;

        Ok(external_link.0)
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests
}
