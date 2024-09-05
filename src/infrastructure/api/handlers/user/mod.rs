//! Users handlers

mod dto;
mod error;

use crate::domain::entities::user::UserId;
use crate::domain::use_cases::user::create_user::CreateUserUseCaseRequest;
use crate::domain::use_cases::user::get_access_token::GetAccessTokenUseCaseRequest;
use crate::domain::use_cases::user::get_user::GetUserUseCaseRequest;
use crate::domain::use_cases::user::get_users::GetUsersUseCaseRequest;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;
use crate::infrastructure::api::extractors::{ExtractRequestId, Path, Query};
use crate::infrastructure::api::handlers::user::dto::{
    CreateUserRequest, GetAccessTokenRequest, GetAccessTokenResponse, GetUserResponse, GetUsersRequest,
    GetUsersResponse, UserResponse,
};
use crate::infrastructure::api::layers::state::SharedState;
use crate::infrastructure::api::response::{ApiError, ApiSuccess};
use crate::infrastructure::api::use_cases::AppUseCases;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use std::str::FromStr;

/// User creation route: POST /api/v1/users
#[instrument(skip(uc), name = "create_user_handler")]
pub async fn create_user(
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<CreateUserRequest>,
) -> Result<ApiSuccess<UserResponse>, ApiError> {
    let response = uc
        .user
        .create_user
        .call(CreateUserUseCaseRequest::try_from(request)?)
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}

/// User access token route: POST /api/v1/token
#[instrument(skip(uc, state), name = "get_access_token_handler")]
pub async fn get_access_token(
    Extension(uc): Extension<AppUseCases>,
    State(state): State<SharedState>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<GetAccessTokenRequest>,
) -> Result<ApiSuccess<GetAccessTokenResponse>, ApiError> {
    let email = Email::new(&request.email).map_err(|err| ApiError::BadRequest(err.to_string()))?;
    let password = Password::new(&request.password, false).map_err(|err| ApiError::BadRequest(err.to_string()))?;

    let response = uc
        .user
        .get_access_token
        .call(GetAccessTokenUseCaseRequest {
            email,
            password,
            jwt: state.jwt.clone(),
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}

/// Users list route: POST /api/v1/users
#[instrument(skip(uc), name = "get_users_handler")]
pub async fn get_users(
    Query(request): Query<GetUsersRequest>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<GetUsersResponse>, ApiError> {
    let response = uc
        .user
        .get_users
        .call(GetUsersUseCaseRequest::try_from(request)?)
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}

/// Users list route: POST /api/v1/users/:user_id
#[instrument(skip(uc), name = "get_user_handler")]
pub async fn get_user(
    Path(user_id): Path<String>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<GetUserResponse>, ApiError> {
    let user_id = UserId::from_str(&user_id)?;

    let response = uc.user.get_user.call(GetUserUseCaseRequest { user_id }).await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}
