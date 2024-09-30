//! Scopes handlers

use crate::domain::use_cases::scope::create_scope::CreateScopeUseCaseRequest;
use crate::infrastructure::api::extractors::ExtractRequestId;
use crate::infrastructure::api::handlers::scope::dto::{CreateScopeRequest, ScopeResponse};
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
