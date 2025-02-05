//! Restore external link use case

use super::ExternalLinkUseCaseError;
use crate::domain::{
    entities::external_link::ExternalLinkId,
    repositories::external_link::{dto::RestoreExternalLinkDtoRequest, ExternalLinkRepository},
};

#[derive(Debug, Clone)]
pub struct RestoreExternalLinkUseCaseRequest {
    pub id: ExternalLinkId,
}

#[derive(Debug, Clone)]
pub struct RestoreExternalLinkUseCaseResponse();

#[derive(Debug, Clone)]
pub struct RestoreExternalLinkUseCase<L: ExternalLinkRepository> {
    external_link_repository: L,
}

impl<L: ExternalLinkRepository> RestoreExternalLinkUseCase<L> {
    /// Create a new use case
    pub fn new(external_link_repository: L) -> Self {
        Self {
            external_link_repository,
        }
    }

    /// Restore an external link
    #[instrument(skip(self), name = "restore_external_link_use_case")]
    pub async fn call(
        &self,
        request: RestoreExternalLinkUseCaseRequest,
    ) -> Result<RestoreExternalLinkUseCaseResponse, ExternalLinkUseCaseError> {
        let external_link = self
            .external_link_repository
            .restore(RestoreExternalLinkDtoRequest(request))
            .await?;

        Ok(external_link.0)
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests
}
