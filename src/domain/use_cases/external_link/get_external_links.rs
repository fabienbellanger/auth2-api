//! Create external link use case

use super::{ExternalLinkUseCaseError, ExternalLinkUseCaseResponse};
use crate::domain::repositories::external_link::dto::{CountExternalLinksDtoRequest, GetExternalLinksDtoRequest};
use crate::domain::repositories::external_link::ExternalLinkRepository;
use crate::domain::value_objects::pagination::Pagination;
use crate::domain::value_objects::query_sort::QuerySorts;

#[derive(Debug, Clone)]
pub struct GetExternalLinksUseCaseRequest {
    pub pagination: Pagination,
    pub sorts: Option<QuerySorts>,
    pub deleted: bool,
}

#[derive(Debug, Clone)]
pub struct GetExternalLinksUseCaseResponse {
    pub external_links: Vec<ExternalLinkUseCaseResponse>,
    pub total: i64,
}

#[derive(Debug, Clone)]
pub struct GetExternalLinksUseCase<L: ExternalLinkRepository> {
    external_link_repository: L,
}

impl<L: ExternalLinkRepository> GetExternalLinksUseCase<L> {
    /// Create a new use case
    pub fn new(external_link_repository: L) -> Self {
        Self {
            external_link_repository,
        }
    }

    /// Get external links
    #[instrument(skip(self), name = "get_external_links_use_case")]
    pub async fn call(
        &self,
        request: GetExternalLinksUseCaseRequest,
    ) -> Result<GetExternalLinksUseCaseResponse, ExternalLinkUseCaseError> {
        let total = self
            .external_link_repository
            .count_external_links(CountExternalLinksDtoRequest {
                deleted: request.deleted,
            })
            .await?
            .0;

        let external_links = self
            .external_link_repository
            .get_external_links(GetExternalLinksDtoRequest(request))
            .await?
            .0;

        Ok(GetExternalLinksUseCaseResponse { external_links, total })
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests
}
