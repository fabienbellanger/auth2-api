//! Mock of the password reset repository

use crate::domain::repositories::password_reset::dto::{
    CreateUpdatePasswordResetDtoRequest, CreateUpdatePasswordResetDtoResponse, DeletePasswordResetDtoRequest,
    DeletePasswordResetDtoResponse, GetUserIdFromTokenDtoRequest, GetUserIdFromTokenDtoResponse,
};
use crate::domain::repositories::password_reset::PasswordResetRepository;
use crate::domain::use_cases::user::UserUseCaseError;
use async_trait::async_trait;

/// Password reset repository mock
#[derive(Debug, Clone)]
pub struct PasswordResetRepositoryMock {}

#[async_trait]
impl PasswordResetRepository for PasswordResetRepositoryMock {
    /// Add or update forgotten password request
    async fn create_or_update(
        &self,
        _req: CreateUpdatePasswordResetDtoRequest,
    ) -> Result<CreateUpdatePasswordResetDtoResponse, UserUseCaseError> {
        todo!()
    }

    /// Get user ID from token
    async fn get_user_from_token(
        &self,
        _req: GetUserIdFromTokenDtoRequest,
    ) -> Result<GetUserIdFromTokenDtoResponse, UserUseCaseError> {
        todo!()
    }

    /// Delete forgotten password
    async fn delete(
        &self,
        _req: DeletePasswordResetDtoRequest,
    ) -> Result<DeletePasswordResetDtoResponse, UserUseCaseError> {
        todo!()
    }
}
