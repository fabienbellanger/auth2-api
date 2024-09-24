//! Users handlers DTO

use crate::domain::use_cases::user::create_user::CreateUserUseCaseRequest;
use crate::domain::use_cases::user::delete_user::DeleteUserUseCaseResponse;
use crate::domain::use_cases::user::forgotten_password::ForgottenPasswordUseCaseResponse;
use crate::domain::use_cases::user::get_access_token::GetAccessTokenUseCaseResponse;
use crate::domain::use_cases::user::get_user::GetUserUseCaseResponse;
use crate::domain::use_cases::user::get_users::GetUsersUseCaseResponse;
use crate::domain::use_cases::user::refresh_token::RefreshTokenUseCaseResponse;
use crate::domain::use_cases::user::restore_user::RestoreUserUseCaseResponse;
use crate::domain::use_cases::user::{UserUseCaseError, UserUseCaseResponse};
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;
use crate::infrastructure::api::handlers::filter::FilterRequest;
use serde::{Deserialize, Serialize};

/// User response
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl From<UserUseCaseResponse> for UserResponse {
    fn from(value: UserUseCaseResponse) -> Self {
        Self {
            id: value.id.to_string(),
            lastname: value.lastname,
            firstname: value.firstname,
            email: value.email.value(),
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
            deleted_at: value.deleted_at.map(|d| d.to_string()),
        }
    }
}

// ================ User creation ================

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub lastname: String,
    pub firstname: String,
}

impl TryFrom<CreateUserRequest> for CreateUserUseCaseRequest {
    type Error = UserUseCaseError;

    fn try_from(value: CreateUserRequest) -> Result<Self, Self::Error> {
        let email = Email::new(&value.email).map_err(|err| UserUseCaseError::InvalidEmail(err.to_string()))?;
        let password =
            Password::new(&value.password, false).map_err(|err| UserUseCaseError::InvalidPassword(err.to_string()))?;

        Ok(Self {
            email,
            password,
            lastname: value.lastname,
            firstname: value.firstname,
        })
    }
}

// ================ Get access token ================

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccessTokenRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetAccessTokenResponse {
    pub access_token: String,
    pub access_token_expired_at: String,
    pub refresh_token: String,
    pub refresh_token_expired_at: String,
}

impl From<GetAccessTokenUseCaseResponse> for GetAccessTokenResponse {
    fn from(value: GetAccessTokenUseCaseResponse) -> Self {
        Self {
            access_token: value.access_token.token,
            access_token_expired_at: value.access_token.expired_at.to_string(),
            refresh_token: value.refresh_token.refresh_token.to_string(),
            refresh_token_expired_at: value.refresh_token.expired_at.to_string(),
        }
    }
}

// ================ Get users ================

pub type GetUsersRequest = FilterRequest;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetUsersResponse {
    pub total: i64,
    pub data: Vec<UserResponse>,
}

impl From<GetUsersUseCaseResponse> for GetUsersResponse {
    fn from(value: GetUsersUseCaseResponse) -> Self {
        Self {
            total: value.total,
            data: value.users.into_iter().map(|u| u.into()).collect(),
        }
    }
}

// ================ Get user ================

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetUserResponse(UserResponse);

impl From<GetUserUseCaseResponse> for GetUserResponse {
    fn from(value: GetUserUseCaseResponse) -> Self {
        Self(value.0.into())
    }
}

// ================ Delete user ================

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserResponse();

impl From<DeleteUserUseCaseResponse> for DeleteUserResponse {
    fn from(_value: DeleteUserUseCaseResponse) -> Self {
        Self()
    }
}

// ================ Refresh token ================

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub access_token_expired_at: String,
    pub refresh_token: String,
    pub refresh_token_expired_at: String,
}

impl From<RefreshTokenUseCaseResponse> for RefreshTokenResponse {
    fn from(value: RefreshTokenUseCaseResponse) -> Self {
        Self {
            access_token: value.access_token.token,
            access_token_expired_at: value.access_token.expired_at.to_string(),
            refresh_token: value.refresh_token.refresh_token.to_string(),
            refresh_token_expired_at: value.refresh_token.expired_at.to_string(),
        }
    }
}

// ================ Forgotten password ================

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ForgottenPasswordResponse {
    pub token: String,
    pub expired_at: String,
}

impl From<ForgottenPasswordUseCaseResponse> for ForgottenPasswordResponse {
    fn from(value: ForgottenPasswordUseCaseResponse) -> Self {
        Self {
            token: value.0.token.to_string(),
            expired_at: value.0.expired_at.to_string(),
        }
    }
}

// ================ Update user password from token ================

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UpdatePasswordFromTokenRequest {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePasswordFromTokenResponse();

// ================ Restore user ================

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RestoreUserResponse();

impl From<RestoreUserUseCaseResponse> for RestoreUserResponse {
    fn from(_value: RestoreUserUseCaseResponse) -> Self {
        Self()
    }
}
