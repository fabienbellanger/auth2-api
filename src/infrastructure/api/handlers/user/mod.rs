//! Users handlers

mod dto;
mod error;

use crate::domain::entities::user::UserId;
use crate::domain::use_cases::user::create_user::CreateUserUseCaseRequest;
use crate::domain::use_cases::user::delete_user::DeleteUserUseCaseRequest;
use crate::domain::use_cases::user::forgotten_password::ForgottenPasswordUseCaseRequest;
use crate::domain::use_cases::user::get_access_token::GetAccessTokenUseCaseRequest;
use crate::domain::use_cases::user::get_user::GetUserUseCaseRequest;
use crate::domain::use_cases::user::get_users::GetUsersUseCaseRequest;
use crate::domain::use_cases::user::refresh_token::RefreshTokenUseCaseRequest;
use crate::domain::use_cases::user::update_password_from_token::UpdatePasswordFromTokenUseCaseRequest;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::id::Id;
use crate::domain::value_objects::password::Password;
use crate::infrastructure::api::extractors::{ExtractRequestId, Path, Query};
use crate::infrastructure::api::handlers::user::dto::*;
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

/// Users list route: GET /api/v1/users
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

/// Get user route: GET /api/v1/users/:user_id
#[instrument(skip(uc), name = "get_user_handler")]
pub async fn get_user(
    Path(user_id): Path<String>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<GetUserResponse>, ApiError> {
    let response = uc
        .user
        .get_user
        .call(GetUserUseCaseRequest {
            user_id: UserId::from_str(&user_id)?,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}

/// Delete user route: DELETE /api/v1/users/:user_id
#[instrument(skip(uc), name = "delete_user_handler")]
pub async fn delete_user(
    Path(user_id): Path<String>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<DeleteUserResponse>, ApiError> {
    let response = uc
        .user
        .delete_user
        .call(DeleteUserUseCaseRequest {
            user_id: UserId::from_str(&user_id)?,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::NO_CONTENT, response.into()))
}

/// Refresh token route: POST /api/v1/refresh-token/:refresh_token
#[instrument(skip(uc, state), name = "refresh_token_user_handler")]
pub async fn refresh_token(
    Path(refresh_token): Path<String>,
    State(state): State<SharedState>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<RefreshTokenResponse>, ApiError> {
    let response = uc
        .user
        .refresh_token
        .call(RefreshTokenUseCaseRequest {
            refresh_token_id: Id::from_str(&refresh_token)?,
            jwt: state.jwt.clone(),
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}

/// Send forgotten password request: POST /api/v1/forgotten-password/:email
#[instrument(skip(uc, state), name = "forgotten_password_handler")]
pub async fn forgotten_password(
    Path(email): Path<String>,
    State(state): State<SharedState>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> Result<ApiSuccess<ForgottenPasswordResponse>, ApiError> {
    let response = uc
        .user
        .forgotten_password
        .call(ForgottenPasswordUseCaseRequest {
            user_email: Email::new(&email)?,
            expiration_duration: state.config.forgotten_password_expiration_duration,
        })
        .await?;

    Ok(ApiSuccess::new(StatusCode::OK, response.into()))
}

/// Update user password from forgotten password request: PATCH /api/v1/update-password
#[instrument(skip(uc), name = "update_password_from_token_handler")]
pub async fn update_password_from_token(
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(body): Json<UpdatePasswordFromTokenRequest>,
) -> Result<ApiSuccess<UpdatePasswordFromTokenResponse>, ApiError> {
    uc.user
        .update_password_from_token
        .call(UpdatePasswordFromTokenUseCaseRequest {
            token: body.token,
            password: Password::new(&body.password, false)?,
        })
        .await?;

    Ok(ApiSuccess::new(
        StatusCode::NO_CONTENT,
        UpdatePasswordFromTokenResponse(),
    ))
}
