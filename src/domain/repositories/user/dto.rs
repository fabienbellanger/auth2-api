//! DTO for user repository

use crate::domain::entities::user::UserId;
use crate::domain::use_cases::user::create_user::CreateUserUseCaseRequest;
use crate::domain::use_cases::user::delete_user::{DeleteUserUseCaseRequest, DeleteUserUseCaseResponse};
use crate::domain::use_cases::user::forgotten_password::ForgottenPasswordUseCaseRequest;
use crate::domain::use_cases::user::get_user::GetUserUseCaseRequest;
use crate::domain::use_cases::user::get_users::GetUsersUseCaseRequest;
use crate::domain::use_cases::user::UserUseCaseResponse;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;

// ================ User creation ================

/// Create user request
#[derive(Debug, Clone)]
pub struct CreateUserDtoRequest(pub CreateUserUseCaseRequest);

/// Create user response
#[derive(Debug, Clone)]
pub struct CreateUserDtoResponse(pub UserUseCaseResponse);

// ================ Get access token ================

/// Get user information for access token generation request
#[derive(Debug, Clone)]
pub struct GetAccessTokenInformationDtoRequest(pub Email);

/// Get user information for access token generation response
#[derive(Debug, Clone)]
pub struct GetAccessTokenInformationDtoResponse {
    /// User ID
    pub id: UserId,

    /// User password
    pub password: Password,
}

// ================ Get users ================

/// Count users request
#[derive(Debug, Clone)]
pub struct CountUsersDtoRequest {
    pub deleted: bool,
}

/// Count users response
#[derive(Debug, Clone)]
pub struct CountUsersDtoResponse(pub i64);

/// Get users request
#[derive(Debug, Clone)]
pub struct GetUsersDtoRequest(pub GetUsersUseCaseRequest);

/// Get users response
#[derive(Debug, Clone)]
pub struct GetUsersDtoResponse(pub Vec<UserUseCaseResponse>);

// ================ Get a user by ID ================

/// Get user by ID request
#[derive(Debug, Clone)]
pub struct GetUserByIdDtoRequest(pub GetUserUseCaseRequest);

/// Get user by ID response
#[derive(Debug, Clone)]
pub struct GetUserByIdDtoResponse(pub UserUseCaseResponse);

// ================ Get a user by email ================

/// Get user by email request
#[derive(Debug, Clone)]
pub struct GetUserByEmailDtoRequest(pub ForgottenPasswordUseCaseRequest);

/// Get user by email response
#[derive(Debug, Clone)]
pub struct GetUserByEmailDtoResponse(pub UserUseCaseResponse);

// ================ Delete a user ================

/// Delete a user request
#[derive(Debug, Clone)]
pub struct DeleteUserDtoRequest(pub DeleteUserUseCaseRequest);

/// Delete a user response
#[derive(Debug, Clone)]
pub struct DeleteUserDtoResponse(pub DeleteUserUseCaseResponse);

// ================ Update password ================

/// Update user password request
#[derive(Debug, Clone)]
pub struct UpdatePasswordDtoRequest {
    /// User ID
    pub user_id: UserId,

    /// New password
    pub password: Password,
}

/// Update user password response
#[derive(Debug, Clone)]
pub struct UpdatePasswordDtoResponse();
