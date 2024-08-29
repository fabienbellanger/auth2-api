//! Mock of the user repository

use crate::domain::entities::user::UserId;
use crate::domain::repositories::user::dto::{CreateUserDtoRequest, CreateUserDtoResponse};
use crate::domain::repositories::user::UserRepository;
use crate::domain::use_cases::user::create_user::{CreateUserUseCaseResponse, UserCreationError};
use crate::domain::value_objects::datetime::UtcDateTime;
use async_trait::async_trait;
use std::str::FromStr;

pub const VALID_ID: &str = "3288fb86-db99-471d-95bc-1451c7ec6f7b";
pub const VALID_EMAIL: &str = "john.doe@test.com";
pub const INVALID_ID: &str = "1a811ea9-2c02-4acc-ae9f-c8f8522702f3";
pub const INVALID_EMAIL: &str = "jane.doe@test.com";

/// User repository mock
#[derive(Debug, Clone)]
pub struct UserRepositoryMock {}

#[async_trait]
impl UserRepository for UserRepositoryMock {
    /// Create a new user
    ///
    /// If the email is VALID_EMAIL, return a valid user, else return an error
    async fn create_user(&self, req: CreateUserDtoRequest) -> Result<CreateUserDtoResponse, UserCreationError> {
        if req.0.email.value() == VALID_EMAIL {
            Ok(CreateUserDtoResponse(CreateUserUseCaseResponse {
                id: UserId::from_str(VALID_ID).unwrap(),
                email: req.0.email,
                password: req.0.password,
                lastname: req.0.lastname,
                firstname: req.0.firstname,
                created_at: UtcDateTime::now(),
                updated_at: UtcDateTime::now(),
            }))
        } else {
            Err(UserCreationError::UserAlreadyExists(req.0.email.value()))
        }
    }
}
