//! Password reset repository

use crate::domain::repositories::password_reset::dto::{
    CreateUpdatePasswordResetDtoRequest, CreateUpdatePasswordResetDtoResponse, DeletePasswordResetDtoRequest,
    DeletePasswordResetDtoResponse, GetUserIdFromTokenDtoRequest, GetUserIdFromTokenDtoResponse,
};
use crate::domain::use_cases::user::UserUseCaseError;
use async_trait::async_trait;

pub mod dto;

#[async_trait]
pub trait PasswordResetRepository {
    /// Add or update forgotten password request
    async fn create_or_update(
        &self,
        request: CreateUpdatePasswordResetDtoRequest,
    ) -> Result<CreateUpdatePasswordResetDtoResponse, UserUseCaseError>;

    /// Get user ID from token
    async fn get_user_from_token(
        &self,
        request: GetUserIdFromTokenDtoRequest,
    ) -> Result<GetUserIdFromTokenDtoResponse, UserUseCaseError>;

    /// Delete forgotten password
    async fn delete(
        &self,
        request: DeletePasswordResetDtoRequest,
    ) -> Result<DeletePasswordResetDtoResponse, UserUseCaseError>;
}
