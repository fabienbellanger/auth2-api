//! Applications handlers

use crate::domain::use_cases::application::create_application::CreateApplicationUseCaseRequest;
use crate::domain::use_cases::application::get_application::GetApplicationByIdUseCaseRequest;
use crate::domain::use_cases::application::get_applications::GetApplicationsUseCaseRequest;
use crate::domain::value_objects::id::Id;
use crate::infrastructure::api::extractors::{ExtractRequestId, Path, Query};
use crate::infrastructure::api::handlers::application::dto::*;
use crate::infrastructure::api::response::{ApiError, ApiSuccess};
use crate::infrastructure::api::use_cases::AppUseCases;
use axum::http::StatusCode;
use axum::{Extension, Json};
use std::str::FromStr;

mod dto;
mod error;

/// Application creation route: POST /api/v1/applications
#[instrument(skip(uc), name = "create_application_handler")]
pub async fn create(
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<CreateApplicationRequest>,
) -> Result<ApiSuccess<ApplicationResponse>, ApiError> {
    let response = uc
        .application
        .create_application
        .call(CreateApplicationUseCaseRequest::from(request))
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}

/// Get application by ID route: GET /api/v1/applications/:application_id
#[instrument(skip(uc), name = "get_application_by_id_handler")]
pub async fn get_by_id(
    Path(application_id): Path<String>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<ApplicationResponse>, ApiError> {
    let response = uc
        .application
        .get_application
        .call(GetApplicationByIdUseCaseRequest {
            id: Id::from_str(&application_id)?,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}

/// Get applications route: GET /api/v1/applications
#[instrument(skip(uc), name = "get_applications_handler")]
pub async fn get_all(
    Query(request): Query<GetApplicationsRequest>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<GetApplicationsResponse>, ApiError> {
    let response = uc
        .application
        .get_applications
        .call(GetApplicationsUseCaseRequest::try_from(request)?)
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}
