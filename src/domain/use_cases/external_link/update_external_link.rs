//! Update external link use case

use super::ExternalLinkUseCaseError;
use crate::domain::{
    entities::external_link::ExternalLinkId,
    repositories::external_link::{dto::UpdateExternalLinkDtoRequest, ExternalLinkRepository},
};

#[derive(Debug, Clone)]
pub struct UpdateExternalLinkUseCaseRequest {
    pub id: ExternalLinkId,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct UpdateExternalLinkUseCaseResponse();

#[derive(Debug, Clone)]
pub struct UpdateExternalLinkUseCase<L: ExternalLinkRepository> {
    external_link_repository: L,
}

impl<L: ExternalLinkRepository> UpdateExternalLinkUseCase<L> {
    /// Create a new use case
    pub fn new(external_link_repository: L) -> Self {
        Self {
            external_link_repository,
        }
    }

    /// Update an external link
    #[instrument(skip(self), name = "update_external_link_use_case")]
    pub async fn call(
        &self,
        request: UpdateExternalLinkUseCaseRequest,
    ) -> Result<UpdateExternalLinkUseCaseResponse, ExternalLinkUseCaseError> {
        let external_link = self
            .external_link_repository
            .update(UpdateExternalLinkDtoRequest(request))
            .await?;

        Ok(external_link.0)
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
