//! DTO for user repository

use crate::domain::entities::user::UserId;
use crate::domain::use_cases::user::create_user::CreateUserUseCaseRequest;
use crate::domain::use_cases::user::get_users::GetUsersUseCaseRequest;
use crate::domain::use_cases::user::UserUseCaseResponse;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;

/// Create user request
#[derive(Debug, Clone)]
pub struct CreateUserDtoRequest(pub CreateUserUseCaseRequest);

/// Create user response
#[derive(Debug, Clone)]
pub struct CreateUserDtoResponse(pub UserUseCaseResponse);

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

/// Count users request
pub type CountUsersDtoRequest = ();

/// Count users response
#[derive(Debug, Clone)]
pub struct CountUsersDtoResponse(pub i64);

/// Get users request
#[derive(Debug, Clone)]
pub struct GetUsersDtoRequest(pub GetUsersUseCaseRequest);

/// Get users response
#[derive(Debug, Clone)]
pub struct GetUsersDtoResponse(pub Vec<UserUseCaseResponse>);
