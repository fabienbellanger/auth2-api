//! User repository

pub mod dto;

use crate::domain::repositories::user::dto::{
    CountUsersDtoRequest, CountUsersDtoResponse, CreateUserDtoRequest, CreateUserDtoResponse, DeleteUserDtoRequest,
    DeleteUserDtoResponse, GetAccessTokenInformationDtoRequest, GetAccessTokenInformationDtoResponse,
    GetUserByIdDtoRequest, GetUserByIdDtoResponse, GetUsersDtoRequest, GetUsersDtoResponse,
};
use crate::domain::use_cases::user::UserUseCaseError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Clone {
    /// Create a new user
    async fn create_user(&self, req: CreateUserDtoRequest) -> Result<CreateUserDtoResponse, UserUseCaseError>;

    /// Get user information for access token generation
    async fn get_access_token_information(
        &self,
        req: GetAccessTokenInformationDtoRequest,
    ) -> Result<Option<GetAccessTokenInformationDtoResponse>, UserUseCaseError>;

    /// Get all users
    async fn get_users(&self, req: GetUsersDtoRequest) -> Result<GetUsersDtoResponse, UserUseCaseError>;

    /// Count all users
    async fn count_users(&self, req: CountUsersDtoRequest) -> Result<CountUsersDtoResponse, UserUseCaseError>;

    /// Get a user by ID
    async fn get_user_by_id(&self, req: GetUserByIdDtoRequest) -> Result<GetUserByIdDtoResponse, UserUseCaseError>;

    /// Delete a user by ID
    async fn delete_user(&self, req: DeleteUserDtoRequest) -> Result<DeleteUserDtoResponse, UserUseCaseError>;
}
