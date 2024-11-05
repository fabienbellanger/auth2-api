//! External link repository

use crate::domain::use_cases::external_link::ExternalLinkUseCaseError;
use async_trait::async_trait;
use dto::{CreateExternalLinkDtoRequest, CreateExternalLinkDtoResponse};

pub mod dto;

#[async_trait]
pub trait ExternalLinkRepository: Clone {
    /// Create external link
    async fn create(
        &self,
        req: CreateExternalLinkDtoRequest,
    ) -> Result<CreateExternalLinkDtoResponse, ExternalLinkUseCaseError>;
    /*
        /// Get external link by ID
        async fn get_by_id(
            &self,
            req: GetExternalLinkByIdDtoRequest,
        ) -> Result<GetExternalLinkByIdDtoResponse, ExternalLinkUseCaseError>;

        /// Get all external links
        async fn get_external_links(
            &self,
            req: GetExternalLinksDtoRequest,
        ) -> Result<GetExternalLinksDtoResponse, ExternalLinkUseCaseError>;

        /// Update external link
        async fn update(
            &self,
            req: UpdateExternalLinkDtoRequest,
        ) -> Result<UpdateExternalLinkDtoResponse, ExternalLinkUseCaseError>;

        /// Delete external link
        async fn delete(
            &self,
            req: DeleteExternalLinkDtoRequest,
        ) -> Result<DeleteExternalLinkDtoResponse, ExternalLinkUseCaseError>;

        /// Count all external links
        async fn count_external_links(
            &self,
            req: CountExternalLinksDtoRequest,
        ) -> Result<CountExternalLinksDtoResponse, ExternalLinkUseCaseError>;

        /// Restore deleted external link
        async fn restore(
            &self,
            req: RestoreExternalLinkDtoRequest,
        ) -> Result<RestoreExternalLinkDtoResponse, ExternalLinkUseCaseError>;
    */
}
