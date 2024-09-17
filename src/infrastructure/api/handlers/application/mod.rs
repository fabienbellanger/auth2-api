//! Applications handlers

use crate::domain::use_cases::application::create_application::CreateApplicationUseCaseRequest;
use crate::infrastructure::api::extractors::ExtractRequestId;
use crate::infrastructure::api::handlers::application::dto::*;
use crate::infrastructure::api::response::{ApiError, ApiSuccess};
use crate::infrastructure::api::use_cases::AppUseCases;
use axum::http::StatusCode;
use axum::{Extension, Json};

mod dto;
mod error;

/// Application creation route: POST /api/v1/applications
#[instrument(skip(uc), name = "create_application_handler")]
pub async fn create_application(
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
