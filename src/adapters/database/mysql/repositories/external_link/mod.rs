//! External link MySQL repository

mod model;

use crate::{
    adapters::database::mysql::Db,
    domain::{
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
        use_cases::external_link::ExternalLinkUseCaseError,
    },
};
use async_trait::async_trait;
use std::sync::Arc;

/// External link MySQL repository
#[derive(Debug, Clone)]
pub struct ExternalLinkMysqlRepository {
    db: Arc<Db>,
}

impl ExternalLinkMysqlRepository {
    /// Create a new repository
    pub fn new(db: Db) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl ExternalLinkRepository for ExternalLinkMysqlRepository {
    #[instrument(skip(self), name = "external_link_repository_create")]
    async fn create(
        &self,
        _req: CreateExternalLinkDtoRequest,
    ) -> Result<CreateExternalLinkDtoResponse, ExternalLinkUseCaseError> {
        todo!()
    }

    #[instrument(skip(self), name = "external_link_repository_count")]
    async fn count_external_links(
        &self,
        _req: CountExternalLinksDtoRequest,
    ) -> Result<CountExternalLinksDtoResponse, ExternalLinkUseCaseError> {
        todo!()
    }

    #[instrument(skip(self), name = "external_link_repository_get_all")]
    async fn get_external_links(
        &self,
        _req: GetExternalLinksDtoRequest,
    ) -> Result<GetExternalLinksDtoResponse, ExternalLinkUseCaseError> {
        todo!()
    }

    #[instrument(skip(self), name = "external_link_repository_get_by_id")]
    async fn get_by_id(
        &self,
        _req: GetExternalLinkByIdDtoRequest,
    ) -> Result<GetExternalLinkByIdDtoResponse, ExternalLinkUseCaseError> {
        todo!()
    }

    #[instrument(skip(self), name = "external_link_repository_update")]
    async fn update(
        &self,
        _req: UpdateExternalLinkDtoRequest,
    ) -> Result<UpdateExternalLinkDtoResponse, ExternalLinkUseCaseError> {
        todo!()
    }

    #[instrument(skip(self), name = "external_link_repository_delete")]
    async fn delete(
        &self,
        _req: DeleteExternalLinkDtoRequest,
    ) -> Result<DeleteExternalLinkDtoResponse, ExternalLinkUseCaseError> {
        todo!()
    }

    #[instrument(skip(self), name = "external_link_repository_restore")]
    async fn restore(
        &self,
        _req: RestoreExternalLinkDtoRequest,
    ) -> Result<RestoreExternalLinkDtoResponse, ExternalLinkUseCaseError> {
        todo!()
    }
}
