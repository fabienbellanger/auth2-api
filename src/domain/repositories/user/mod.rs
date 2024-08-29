//! User repository

pub mod dto;

use crate::domain::repositories::user::dto::{CreateUserDtoRequest, CreateUserDtoResponse};
use crate::domain::use_cases::user::create_user::UserCreationError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Clone {
    /// Create a new user
    async fn create_user(&self, req: CreateUserDtoRequest) -> Result<CreateUserDtoResponse, UserCreationError>;
}
