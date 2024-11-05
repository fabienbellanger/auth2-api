//! Create external link use case

use super::{ExternalLinkUseCaseError, ExternalLinkUseCaseResponse};
use crate::domain::entities::external_link::ExternalLinkId;
use crate::domain::repositories::external_link::dto::CreateExternalLinkDtoRequest;
use crate::domain::repositories::external_link::ExternalLinkRepository;

#[derive(Debug, Clone)]
pub struct CreateExternalLinkUseCaseRequest {
    pub id: ExternalLinkId,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct CreateExternalLinkUseCase<L: ExternalLinkRepository> {
    external_link_repository: L,
}

impl<L: ExternalLinkRepository> CreateExternalLinkUseCase<L> {
    /// Create a new use case
    pub fn new(external_link_repository: L) -> Self {
        Self {
            external_link_repository,
        }
    }

    /// Create a new external link
    #[instrument(skip(self), name = "create_external_link_use_case")]
    pub async fn call(
        &self,
        request: CreateExternalLinkUseCaseRequest,
    ) -> Result<ExternalLinkUseCaseResponse, ExternalLinkUseCaseError> {
        let external_link = self
            .external_link_repository
            .create(CreateExternalLinkDtoRequest(request))
            .await?;

        Ok(external_link.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        tests::mock::external_link::{ExternalLinkRepositoryMock, VALID_EXTERNAL_LINK_ID, VALID_EXTERNAL_LINK_NAME},
        value_objects::id::Id,
    };
    use std::str::FromStr;

    #[tokio::test]
    async fn test_create_external_link() {
        let repository = ExternalLinkRepositoryMock {};
        let create_external_link = CreateExternalLinkUseCase::new(repository);

        let result = create_external_link
            .call(CreateExternalLinkUseCaseRequest {
                id: Id::from_str(VALID_EXTERNAL_LINK_ID).unwrap(),
                name: VALID_EXTERNAL_LINK_NAME.to_string(),
            })
            .await;

        assert!(result.is_ok());
    }
}