//! Applications handlers

use crate::domain::use_cases::application::create_application::CreateApplicationUseCaseRequest;
use crate::domain::use_cases::application::delete_application::DeleteApplicationUseCaseRequest;
use crate::domain::use_cases::application::get_application::GetApplicationByIdUseCaseRequest;
use crate::domain::use_cases::application::get_applications::GetApplicationsUseCaseRequest;
use crate::domain::use_cases::application::update_application::UpdateApplicationUseCaseRequest;
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

    Ok(ApiSuccess::new(StatusCode::CREATED, response.into()))
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
        .call(GetApplicationsUseCaseRequest {
            filter: request.try_into()?,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}

/// Delete an application route: DELETE /api/v1/applications/:application_id
#[instrument(skip(uc), name = "delete_application_handler")]
pub async fn delete(
    Path(application_id): Path<String>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<DeleteApplicationResponse>, ApiError> {
    let response = uc
        .application
        .delete_application
        .call(DeleteApplicationUseCaseRequest {
            id: Id::from_str(&application_id)?,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::NO_CONTENT, response.into()))
}

/// Update an application route: PATCH /api/v1/applications/:application_id
#[instrument(skip(uc), name = "update_application_handler")]
pub async fn update(
    Path(application_id): Path<String>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<UpdateApplicationRequest>,
) -> Result<ApiSuccess<UpdateApplicationResponse>, ApiError> {
    let response = uc
        .application
        .update_application
        .call(UpdateApplicationUseCaseRequest {
            id: Id::from_str(&application_id)?,
            name: request.name,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::NO_CONTENT, response.into()))
}
