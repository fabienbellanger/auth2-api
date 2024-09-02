//! Users handlers

mod dto;

use crate::domain::use_cases::user::create_user::CreateUserUseCaseRequest;
use crate::infrastructure::api::extractors::ExtractRequestId;
use crate::infrastructure::api::handlers::user::dto::{CreateUserRequest, CreateUserResponse};
use crate::infrastructure::api::response::{ApiError, ApiSuccess};
use crate::infrastructure::api::user_cases::AppUseCases;
use axum::http::StatusCode;
use axum::{Extension, Json};

/// User creation route: POST /api/v1/users
#[instrument(skip(uc), name = "create_user_handler")]
pub async fn create_user(
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<CreateUserRequest>,
) -> Result<ApiSuccess<CreateUserResponse>, ApiError> {
    let response = uc
        .user
        .create_user
        .call(CreateUserUseCaseRequest::try_from(request)?)
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}
