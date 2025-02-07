//! External links handlers

mod dto;
mod error;

use crate::domain::use_cases::external_link::create_external_link::CreateExternalLinkUseCaseRequest;
use crate::domain::use_cases::external_link::delete_external_link::DeleteExternalLinkUseCaseRequest;
use crate::domain::use_cases::external_link::get_external_link::GetExternalLinkByIdUseCaseRequest;
use crate::domain::use_cases::external_link::get_external_links::GetExternalLinksUseCaseRequest;
use crate::domain::use_cases::external_link::restore_external_link::RestoreExternalLinkUseCaseRequest;
use crate::domain::use_cases::external_link::update_external_link::UpdateExternalLinkUseCaseRequest;
use crate::domain::value_objects::id::Id;
use crate::infrastructure::api::extractors::{ExtractRequestId, Path, Query};
use crate::infrastructure::api::response::{ApiError, ApiSuccess};
use crate::infrastructure::api::use_cases::AppUseCases;
use axum::http::StatusCode;
use axum::{Extension, Json};
use dto::{
    CreateExternalLinkRequest, DeleteExternalLinkResponse, ExternalLinkResponse, GetExternalLinksRequest,
    GetExternalLinksResponse, RestoreExternalLinkResponse, UpdateExternalLinkRequest, UpdateExternalLinkResponse,
};
use std::str::FromStr;

/// External link creation route: POST /api/v1/external-links
#[instrument(skip(uc), name = "create_external_link_handler")]
pub async fn create(
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<CreateExternalLinkRequest>,
) -> Result<ApiSuccess<ExternalLinkResponse>, ApiError> {
    let response = uc
        .external_link
        .create_external_link
        .call(CreateExternalLinkUseCaseRequest::from(request))
        .await?;

    Ok(ApiSuccess::new(StatusCode::CREATED, response.into()))
}

/// Get external link by ID route: GET /api/v1/external-links/:external_link_id
#[instrument(skip(uc), name = "get_external_link_by_id_handler")]
pub async fn get_by_id(
    Path(external_link_id): Path<String>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<ExternalLinkResponse>, ApiError> {
    let response = uc
        .external_link
        .get_external_link
        .call(GetExternalLinkByIdUseCaseRequest {
            id: Id::from_str(&external_link_id)?,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}

/// Get external links route: GET /api/v1/external-links
#[instrument(skip(uc), name = "get_external_links_handler")]
pub async fn get_all(
    Query(request): Query<GetExternalLinksRequest>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<GetExternalLinksResponse>, ApiError> {
    let response = uc
        .external_link
        .get_external_links
        .call(GetExternalLinksUseCaseRequest {
            pagination: request.pagination(),
            sorts: request.sorts(),
            deleted: false,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}

/// Get deleted external links route: GET /api/v1/external-links/deleted
#[instrument(skip(uc), name = "get_deleted_external_links_handler")]
pub async fn get_all_deleted(
    Query(request): Query<GetExternalLinksRequest>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<GetExternalLinksResponse>, ApiError> {
    let response = uc
        .external_link
        .get_external_links
        .call(GetExternalLinksUseCaseRequest {
            pagination: request.pagination(),
            sorts: request.sorts(),
            deleted: true,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}

/// Delete an external link route: DELETE /api/v1/external-links/:external_link_id
#[instrument(skip(uc), name = "delete_external_link_handler")]
pub async fn delete(
    Path(external_link_id): Path<String>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<DeleteExternalLinkResponse>, ApiError> {
    let response = uc
        .external_link
        .delete_external_link
        .call(DeleteExternalLinkUseCaseRequest {
            id: Id::from_str(&external_link_id)?,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::NO_CONTENT, response.into()))
}

/// Update an external link route: PATCH /api/v1/external-links/:external_link_id
#[instrument(skip(uc), name = "update_external_link_handler")]
pub async fn update(
    Path(external_link_id): Path<String>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<UpdateExternalLinkRequest>,
) -> Result<ApiSuccess<UpdateExternalLinkResponse>, ApiError> {
    let response = uc
        .external_link
        .update_external_link
        .call(UpdateExternalLinkUseCaseRequest {
            id: Id::from_str(&external_link_id)?,
            name: request.name,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::NO_CONTENT, response.into()))
}

/// Restore a deleted external link route: PATCH /api/v1/external-links/:external_link_id/restore
#[instrument(skip(uc), name = "restore_external_link_handler")]
pub async fn restore(
    Path(external_link_id): Path<String>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<RestoreExternalLinkResponse>, ApiError> {
    let response = uc
        .external_link
        .restore_external_link
        .call(RestoreExternalLinkUseCaseRequest {
            id: Id::from_str(&external_link_id)?,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::NO_CONTENT, response.into()))
}
