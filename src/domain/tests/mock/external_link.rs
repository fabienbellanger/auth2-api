//! Mock of the external link repository

use crate::domain::{
    repositories::external_link::{
        dto::{
            CountExternalLinksDtoRequest, CountExternalLinksDtoResponse, CreateExternalLinkDtoRequest,
            CreateExternalLinkDtoResponse, DeleteExternalLinkDtoRequest, DeleteExternalLinkDtoResponse,
            GetExternalLinkByIdDtoRequest, GetExternalLinkByIdDtoResponse, GetExternalLinksDtoRequest,
            GetExternalLinksDtoResponse, RestoreExternalLinkDtoRequest, RestoreExternalLinkDtoResponse,
            UpdateExternalLinkDtoRequest, UpdateExternalLinkDtoResponse,
        },
        ExternalLinkRepository,
    },
    use_cases::external_link::{ExternalLinkUseCaseError, ExternalLinkUseCaseResponse},
    value_objects::{datetime::UtcDateTime, id::Id},
};
use async_trait::async_trait;
use std::str::FromStr;

pub const VALID_EXTERNAL_LINK_ID: &str = "ffaa2c9c-872f-4e62-8302-d4586096cd13";
pub const INVALID_EXTERNAL_LINK_ID: &str = "b4dc6179-e538-449b-accd-a8a1f58631af";
pub const VALID_EXTERNAL_LINK_NAME: &str = "Test external link";
pub const INVALID_EXTERNAL_LINK_NAME: &str = "Test invalid external link";

/// External link repository mock
#[derive(Debug, Clone)]
pub struct ExternalLinkRepositoryMock {}

#[async_trait]
impl ExternalLinkRepository for ExternalLinkRepositoryMock {
    /// Create external link
    async fn create(
        &self,
        req: CreateExternalLinkDtoRequest,
    ) -> Result<CreateExternalLinkDtoResponse, ExternalLinkUseCaseError> {
        if req.0.name == VALID_EXTERNAL_LINK_NAME {
            let now = UtcDateTime::now();

            Ok(CreateExternalLinkDtoResponse(ExternalLinkUseCaseResponse {
                id: Id::from_str(VALID_EXTERNAL_LINK_ID).unwrap(),
                name: VALID_EXTERNAL_LINK_NAME.to_string(),
                created_at: now.clone(),
                updated_at: now,
                deleted_at: None,
            }))
        } else {
            Err(ExternalLinkUseCaseError::DatabaseError(
                "error during external link creation".to_string(),
            ))
        }
    }

    /// Count all external links
    async fn count_external_links(
        &self,
        _req: CountExternalLinksDtoRequest,
    ) -> Result<CountExternalLinksDtoResponse, ExternalLinkUseCaseError> {
        todo!()
    }

    /// Get all external links
    async fn get_external_links(
        &self,
        _req: GetExternalLinksDtoRequest,
    ) -> Result<GetExternalLinksDtoResponse, ExternalLinkUseCaseError> {
        todo!()
    }

    /// Get external link by ID
    async fn get_by_id(
        &self,
        _req: GetExternalLinkByIdDtoRequest,
    ) -> Result<GetExternalLinkByIdDtoResponse, ExternalLinkUseCaseError> {
        todo!()
    }

    /// Update external link
    async fn update(
        &self,
        _req: UpdateExternalLinkDtoRequest,
    ) -> Result<UpdateExternalLinkDtoResponse, ExternalLinkUseCaseError> {
        todo!()
    }

    /// Delete external link
    async fn delete(
        &self,
        _req: DeleteExternalLinkDtoRequest,
    ) -> Result<DeleteExternalLinkDtoResponse, ExternalLinkUseCaseError> {
        todo!()
    }

    /// Restore deleted external link
    async fn restore(
        &self,
        _req: RestoreExternalLinkDtoRequest,
    ) -> Result<RestoreExternalLinkDtoResponse, ExternalLinkUseCaseError> {
        todo!()
    }
}
