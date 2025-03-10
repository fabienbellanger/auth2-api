//! External links handlers DTO

use crate::{
    domain::use_cases::external_link::{
        ExternalLinkUseCaseResponse, create_external_link::CreateExternalLinkUseCaseRequest,
        delete_external_link::DeleteExternalLinkUseCaseResponse, get_external_links::GetExternalLinksUseCaseResponse,
        restore_external_link::RestoreExternalLinkUseCaseResponse,
        update_external_link::UpdateExternalLinkUseCaseResponse,
    },
    infrastructure::api::handlers::filter::FilterRequest,
};
use serde::{Deserialize, Serialize};

/// External link response
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ExternalLinkResponse {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl From<ExternalLinkUseCaseResponse> for ExternalLinkResponse {
    fn from(value: ExternalLinkUseCaseResponse) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
            deleted_at: value.deleted_at.map(|dt| dt.to_string()),
        }
    }
}

// ================ External link creation ================

#[derive(Debug, Clone, Deserialize)]
pub struct CreateExternalLinkRequest {
    pub name: String,
}

impl From<CreateExternalLinkRequest> for CreateExternalLinkUseCaseRequest {
    fn from(value: CreateExternalLinkRequest) -> Self {
        Self { name: value.name }
    }
}

// ================ Get external links ================

pub type GetExternalLinksRequest = FilterRequest;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetExternalLinksResponse {
    pub total: i64,
    pub data: Vec<ExternalLinkResponse>,
}

impl From<GetExternalLinksUseCaseResponse> for GetExternalLinksResponse {
    fn from(value: GetExternalLinksUseCaseResponse) -> Self {
        Self {
            total: value.total,
            data: value.external_links.into_iter().map(|app| app.into()).collect(),
        }
    }
}

// ================ Delete external link ================

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DeleteExternalLinkResponse();

impl From<DeleteExternalLinkUseCaseResponse> for DeleteExternalLinkResponse {
    fn from(_: DeleteExternalLinkUseCaseResponse) -> Self {
        Self {}
    }
}

// ================ Update external link ================

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateExternalLinkRequest {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UpdateExternalLinkResponse();

impl From<UpdateExternalLinkUseCaseResponse> for UpdateExternalLinkResponse {
    fn from(_: UpdateExternalLinkUseCaseResponse) -> Self {
        Self {}
    }
}

// ================ Restore external link ================

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RestoreExternalLinkResponse();

impl From<RestoreExternalLinkUseCaseResponse> for RestoreExternalLinkResponse {
    fn from(_: RestoreExternalLinkUseCaseResponse) -> Self {
        Self {}
    }
}
