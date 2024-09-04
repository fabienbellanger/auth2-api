//! User repository

pub mod dto;

use crate::domain::repositories::user::dto::{
    CountUsersDtoRequest, CountUsersDtoResponse, CreateUserDtoRequest, CreateUserDtoResponse,
    GetAccessTokenInformationDtoRequest, GetAccessTokenInformationDtoResponse, GetUsersDtoRequest, GetUsersDtoResponse,
};
use crate::domain::use_cases::user::create_user::UserCreationError;
use crate::domain::use_cases::user::get_access_token::GetAccessTokenError;
use crate::domain::use_cases::user::UserUseCaseError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Clone {
    /// Create a new user
    async fn create_user(&self, req: CreateUserDtoRequest) -> Result<CreateUserDtoResponse, UserCreationError>;

    /// Get user information for access token generation
    async fn get_access_token_information(
        &self,
        req: GetAccessTokenInformationDtoRequest,
    ) -> Result<Option<GetAccessTokenInformationDtoResponse>, GetAccessTokenError>;

    /// Get all users
    async fn get_users(&self, req: GetUsersDtoRequest) -> Result<GetUsersDtoResponse, UserUseCaseError>;

    /// Count all users
    async fn count_users(&self, req: CountUsersDtoRequest) -> Result<CountUsersDtoResponse, UserUseCaseError>;
}
