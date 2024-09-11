//! Mock of the user repository

use crate::domain::entities::user::UserId;
use crate::domain::repositories::user::dto::{
    CountUsersDtoRequest, CountUsersDtoResponse, CreateUserDtoRequest, CreateUserDtoResponse, DeleteUserDtoRequest,
    DeleteUserDtoResponse, GetAccessTokenInformationDtoRequest, GetAccessTokenInformationDtoResponse,
    GetUserByEmailDtoRequest, GetUserByEmailDtoResponse, GetUserByIdDtoRequest, GetUserByIdDtoResponse,
    GetUsersDtoRequest, GetUsersDtoResponse, UpdatePasswordDtoRequest, UpdatePasswordDtoResponse,
};
use crate::domain::repositories::user::UserRepository;
use crate::domain::use_cases::user::delete_user::DeleteUserUseCaseResponse;
use crate::domain::use_cases::user::{UserUseCaseError, UserUseCaseResponse};
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;
use async_trait::async_trait;
use std::str::FromStr;

pub const VALID_ID: &str = "3288fb86-db99-471d-95bc-1451c7ec6f7b";
pub const VALID_EMAIL: &str = "john.doe@test.com";
pub const VALID_PASSWORD: &str = "123456789";
pub const INVALID_ID: &str = "1a811ea9-2c02-4acc-ae9f-c8f8522702f3";
pub const INVALID_EMAIL: &str = "jane.doe@test.com";
pub const EMAIL_NOT_FOUND: &str = "lucky.luke@test.com";
pub const INVALID_PASSWORD: &str = "7844125963";

/// User repository mock
#[derive(Debug, Clone)]
pub struct UserRepositoryMock {}

#[async_trait]
impl UserRepository for UserRepositoryMock {
    /// Create a new user
    ///
    /// If the email is VALID_EMAIL, return a valid user, else return an error
    async fn create_user(&self, req: CreateUserDtoRequest) -> Result<CreateUserDtoResponse, UserUseCaseError> {
        if req.0.email.value() == VALID_EMAIL {
            Ok(CreateUserDtoResponse(UserUseCaseResponse {
                id: UserId::from_str(VALID_ID)?,
                email: req.0.email,
                lastname: req.0.lastname,
                firstname: req.0.firstname,
                created_at: UtcDateTime::now(),
                updated_at: UtcDateTime::now(),
            }))
        } else {
            Err(UserUseCaseError::DatabaseError("User creation error".to_string()))
        }
    }

    /// Get a user by email
    async fn get_access_token_information(
        &self,
        req: GetAccessTokenInformationDtoRequest,
    ) -> Result<Option<GetAccessTokenInformationDtoResponse>, UserUseCaseError> {
        match req.0.to_string().as_str() {
            VALID_EMAIL => {
                // Valid user
                Ok(Some(GetAccessTokenInformationDtoResponse {
                    id: UserId::from_str(VALID_ID)?,
                    password: Password::new(VALID_PASSWORD, false)?,
                }))
            }
            EMAIL_NOT_FOUND => Ok(None),
            _ => Err(UserUseCaseError::DatabaseError("User not found".to_string())),
        }
    }

    /// Get all users
    async fn get_users(&self, _req: GetUsersDtoRequest) -> Result<GetUsersDtoResponse, UserUseCaseError> {
        Ok(GetUsersDtoResponse(vec![]))
    }

    /// Count all users
    async fn count_users(&self, _req: CountUsersDtoRequest) -> Result<CountUsersDtoResponse, UserUseCaseError> {
        Ok(CountUsersDtoResponse(0))
    }

    /// Get a user by ID
    async fn get_user_by_id(&self, req: GetUserByIdDtoRequest) -> Result<GetUserByIdDtoResponse, UserUseCaseError> {
        match req.0.user_id.to_string().as_str() {
            VALID_ID => Ok(GetUserByIdDtoResponse(UserUseCaseResponse {
                id: UserId::from_str(VALID_ID)?,
                email: Email::new(VALID_EMAIL)?,
                lastname: "Doe".to_string(),
                firstname: "John".to_string(),
                created_at: UtcDateTime::now(),
                updated_at: UtcDateTime::now(),
            })),
            _ => Err(UserUseCaseError::DatabaseError("User not found".to_string())),
        }
    }

    /// Get a user by email
    async fn get_user_by_email(
        &self,
        _req: GetUserByEmailDtoRequest,
    ) -> Result<GetUserByEmailDtoResponse, UserUseCaseError> {
        todo!()
    }

    /// Delete a user by ID
    async fn delete_user(&self, req: DeleteUserDtoRequest) -> Result<DeleteUserDtoResponse, UserUseCaseError> {
        match req.0.user_id.to_string().as_str() {
            VALID_ID => Ok(DeleteUserDtoResponse(DeleteUserUseCaseResponse())),
            _ => Err(UserUseCaseError::DatabaseError("User not found".to_string())),
        }
    }

    /// Update password
    async fn update_password(
        &self,
        _req: UpdatePasswordDtoRequest,
    ) -> Result<UpdatePasswordDtoResponse, UserUseCaseError> {
        todo!()
    }
}
