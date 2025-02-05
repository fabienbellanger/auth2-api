//! Get an external link by its ID use case

use super::{ExternalLinkUseCaseError, ExternalLinkUseCaseResponse};
use crate::domain::entities::external_link::ExternalLinkId;
use crate::domain::repositories::external_link::dto::GetExternalLinkByIdDtoRequest;
use crate::domain::repositories::external_link::ExternalLinkRepository;

#[derive(Debug, Clone)]
pub struct GetExternalLinkByIdUseCaseRequest {
    pub id: ExternalLinkId,
}

#[derive(Debug, Clone)]
pub struct GetExternalLinkByIdUseCase<L: ExternalLinkRepository> {
    external_link_repository: L,
}

impl<L: ExternalLinkRepository> GetExternalLinkByIdUseCase<L> {
    /// Create a new use case
    pub fn new(external_link_repository: L) -> Self {
        Self {
            external_link_repository,
        }
    }

    /// Get an external link by its ID
    #[instrument(skip(self), name = "get_external_link_use_case")]
    pub async fn call(
        &self,
        request: GetExternalLinkByIdUseCaseRequest,
    ) -> Result<ExternalLinkUseCaseResponse, ExternalLinkUseCaseError> {
        let external_link = self
            .external_link_repository
            .get_by_id(GetExternalLinkByIdDtoRequest(request))
            .await?;

        Ok(external_link.0)
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests
}
