//! Scopes handlers

use crate::domain::use_cases::scope::create_scope::CreateScopeUseCaseRequest;
use crate::domain::use_cases::scope::get_scopes::GetScopesUseCaseRequest;
use crate::infrastructure::api::extractors::{ExtractRequestId, Query};
use crate::infrastructure::api::handlers::scope::dto::{
    CreateScopeRequest, GetScopesRequest, GetScopesResponse, ScopeResponse,
};
use crate::infrastructure::api::response::{ApiError, ApiSuccess};
use crate::infrastructure::api::use_cases::AppUseCases;
use axum::http::StatusCode;
use axum::{Extension, Json};

mod dto;
mod error;

/// Scope creation route: POST /api/v1/scopes
#[instrument(skip(uc), name = "create_scopes_handler")]
pub async fn create(
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<CreateScopeRequest>,
) -> Result<ApiSuccess<ScopeResponse>, ApiError> {
    let response = uc
        .scope
        .create_scope
        .call(CreateScopeUseCaseRequest::try_from(request)?)
        .await?;

    Ok(ApiSuccess::new(StatusCode::CREATED, response.into()))
}

/// Get scopes route: GET /api/v1/scopes
#[instrument(skip(uc), name = "get_scopes_handler")]
pub async fn get_all(
    Query(request): Query<GetScopesRequest>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<GetScopesResponse>, ApiError> {
    let response = uc
        .scope
        .get_scopes
        .call(GetScopesUseCaseRequest {
            pagination: request.pagination(),
            sorts: request.sorts(),
            deleted: false,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}

/// Get scopes route: GET /api/v1/scopes/deleted
#[instrument(skip(uc), name = "get_deleted_scopes_handler")]
pub async fn get_all_deleted(
    Query(request): Query<GetScopesRequest>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<GetScopesResponse>, ApiError> {
    let response = uc
        .scope
        .get_scopes
        .call(GetScopesUseCaseRequest {
            pagination: request.pagination(),
            sorts: request.sorts(),
            deleted: true,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}
