//! User handler DTO

use crate::domain::use_cases::user::create_user::CreateUserUseCaseRequest;
use crate::domain::use_cases::user::delete_user::DeleteUserUseCaseResponse;
use crate::domain::use_cases::user::get_access_token::GetAccessTokenUseCaseResponse;
use crate::domain::use_cases::user::get_user::GetUserUseCaseResponse;
use crate::domain::use_cases::user::get_users::{GetUsersUseCaseRequest, GetUsersUseCaseResponse};
use crate::domain::use_cases::user::{UserUseCaseError, UserUseCaseResponse};
use crate::domain::utils::query_sort::{Filter, Sorts};
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::pagination::Pagination;
use crate::domain::value_objects::password::Password;
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
        }
    }
}

// ================ User creation ================

/// Create user request
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

/// Get access token request
#[derive(Debug, Clone, Deserialize)]
pub struct GetAccessTokenRequest {
    pub email: String,
    pub password: String,
}

/// Get access token response
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

/// Get users request
#[derive(Debug, Clone, Deserialize)]
pub struct GetUsersRequest {
    #[serde(rename(deserialize = "p"))]
    pub page: Option<u32>,

    #[serde(rename(deserialize = "l"))]
    pub limit: Option<u32>,

    #[serde(rename(deserialize = "s"))]
    pub sort: Option<String>,
}

impl TryFrom<GetUsersRequest> for GetUsersUseCaseRequest {
    type Error = UserUseCaseError;

    fn try_from(value: GetUsersRequest) -> Result<Self, Self::Error> {
        let filter = match (value.page, value.limit, value.sort) {
            (Some(page), Some(limit), sort) => {
                let pagination = Some(Pagination::new(page, limit));
                let sorts = sort.map(|sort| Sorts::from(sort.as_str()));

                Some(Filter { pagination, sorts })
            }
            (_, _, Some(sort)) => Some(Filter {
                pagination: None,
                sorts: Some(Sorts::from(sort.as_str())),
            }),
            _ => None,
        };

        Ok(Self { filter })
    }
}

/// Get users response
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

/// Get user response
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetUserResponse(UserResponse);

impl From<GetUserUseCaseResponse> for GetUserResponse {
    fn from(value: GetUserUseCaseResponse) -> Self {
        Self(value.0.into())
    }
}

// ================ Delete user ================

/// Delete user response
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserResponse();

impl From<DeleteUserUseCaseResponse> for DeleteUserResponse {
    fn from(_value: DeleteUserUseCaseResponse) -> Self {
        Self()
    }
}
