//! DTO for user repository

use crate::domain::entities::user::UserId;
use crate::domain::use_cases::user::create_user::CreateUserUseCaseRequest;
use crate::domain::use_cases::user::delete_user::{DeleteUserUseCaseRequest, DeleteUserUseCaseResponse};
use crate::domain::use_cases::user::forgotten_password::ForgottenPasswordUseCaseRequest;
use crate::domain::use_cases::user::get_user::GetUserUseCaseRequest;
use crate::domain::use_cases::user::get_users::GetUsersUseCaseRequest;
use crate::domain::use_cases::user::restore_user::{RestoreUserUseCaseRequest, RestoreUserUseCaseResponse};
use crate::domain::use_cases::user::UserUseCaseResponse;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;

// ================ User creation ================

#[derive(Debug, Clone)]
pub struct CreateUserDtoRequest(pub CreateUserUseCaseRequest);

#[derive(Debug, Clone)]
pub struct CreateUserDtoResponse(pub UserUseCaseResponse);

// ================ Get access token ================

#[derive(Debug, Clone)]
pub struct GetAccessTokenInformationDtoRequest(pub Email);

#[derive(Debug, Clone)]
pub struct GetAccessTokenInformationDtoResponse {
    pub id: UserId,
    pub password: Password,
}

// ================ Get users ================

#[derive(Debug, Clone)]
pub struct CountUsersDtoRequest {
    pub deleted: bool,
}

#[derive(Debug, Clone)]
pub struct CountUsersDtoResponse(pub i64);

#[derive(Debug, Clone)]
pub struct GetUsersDtoRequest(pub GetUsersUseCaseRequest);

#[derive(Debug, Clone)]
pub struct GetUsersDtoResponse(pub Vec<UserUseCaseResponse>);

// ================ Get a user by ID ================

#[derive(Debug, Clone)]
pub struct GetUserByIdDtoRequest(pub GetUserUseCaseRequest);

#[derive(Debug, Clone)]
pub struct GetUserByIdDtoResponse(pub UserUseCaseResponse);

// ================ Get a user by email ================

#[derive(Debug, Clone)]
pub struct GetUserByEmailDtoRequest(pub ForgottenPasswordUseCaseRequest);

#[derive(Debug, Clone)]
pub struct GetUserByEmailDtoResponse(pub UserUseCaseResponse);

// ================ Delete a user ================

#[derive(Debug, Clone)]
pub struct DeleteUserDtoRequest(pub DeleteUserUseCaseRequest);

#[derive(Debug, Clone)]
pub struct DeleteUserDtoResponse(pub DeleteUserUseCaseResponse);

// ================ Update password ================

#[derive(Debug, Clone)]
pub struct UpdatePasswordDtoRequest {
    pub user_id: UserId,
    pub password: Password,
}

#[derive(Debug, Clone)]
pub struct UpdatePasswordDtoResponse();

// ================ Restore a user ================

#[derive(Debug, Clone)]
pub struct RestoreUserDtoRequest(pub RestoreUserUseCaseRequest);

#[derive(Debug, Clone)]
pub struct RestoreUserDtoResponse(pub RestoreUserUseCaseResponse);
