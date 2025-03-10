//! Mock of the password reset repository

use crate::domain::repositories::password_reset::PasswordResetRepository;
use crate::domain::repositories::password_reset::dto::{
    CreateUpdatePasswordResetDtoRequest, CreateUpdatePasswordResetDtoResponse, DeletePasswordResetDtoRequest,
    DeletePasswordResetDtoResponse, GetUserIdFromTokenDtoRequest, GetUserIdFromTokenDtoResponse,
};
use crate::domain::use_cases::user::UserUseCaseError;
use crate::domain::value_objects::id::Id;
use async_trait::async_trait;
use std::str::FromStr;

pub const VALID_TOKEN: &str = "valid_token";
pub const INVALID_TOKEN: &str = "invalid_token";
pub const VALID_USER_ID: &str = "3288fb86-db99-471d-95bc-1451c7ec6f7b";
pub const INVALID_USER_ID: &str = "1a811ea9-2c02-4acc-ae9f-c8f8522702f3";

/// Password reset repository mock
#[derive(Debug, Clone)]
pub struct PasswordResetRepositoryMock {}

#[async_trait]
impl PasswordResetRepository for PasswordResetRepositoryMock {
    /// Add or update forgotten password request
    async fn create_or_update(
        &self,
        req: CreateUpdatePasswordResetDtoRequest,
    ) -> Result<CreateUpdatePasswordResetDtoResponse, UserUseCaseError> {
        if req.0.token.as_str() == VALID_TOKEN {
            Ok(CreateUpdatePasswordResetDtoResponse())
        } else {
            Err(UserUseCaseError::DatabaseError(
                "Failed to create or update password reset".to_string(),
            ))
        }
    }

    /// Get user ID from token
    async fn get_user_from_token(
        &self,
        req: GetUserIdFromTokenDtoRequest,
    ) -> Result<GetUserIdFromTokenDtoResponse, UserUseCaseError> {
        if req.token.as_str() == VALID_TOKEN {
            Ok(GetUserIdFromTokenDtoResponse {
                user_id: Some(Id::from_str(VALID_USER_ID)?),
            })
        } else {
            Ok(GetUserIdFromTokenDtoResponse { user_id: None })
        }
    }

    /// Delete forgotten password
    async fn delete(
        &self,
        req: DeletePasswordResetDtoRequest,
    ) -> Result<DeletePasswordResetDtoResponse, UserUseCaseError> {
        if req.user_id.to_string().as_str() == VALID_USER_ID {
            Ok(DeletePasswordResetDtoResponse())
        } else {
            Err(UserUseCaseError::DatabaseError(
                "Failed to delete password reset".to_string(),
            ))
        }
    }
}
