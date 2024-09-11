//! DTO for password reset repository

use crate::domain::entities::password_reset::{PasswordReset, PasswordResetTokenValue};
use crate::domain::entities::user::UserId;

// ================ Password reset creation or update ================

/// Create or update password reset request
#[derive(Debug, Clone)]
pub struct CreateUpdatePasswordResetDtoRequest(pub PasswordReset);

/// Create or update password reset response
#[derive(Debug, Clone)]
pub struct CreateUpdatePasswordResetDtoResponse();

// ================ Get user ID from password reset token ================

/// Get user ID from password reset token request
#[derive(Debug, Clone)]
pub struct GetUserIdFromTokenDtoRequest {
    pub token: PasswordResetTokenValue,
}

/// Get user ID from password reset token response
#[derive(Debug, Clone)]
pub struct GetUserIdFromTokenDtoResponse {
    pub user_id: Option<UserId>,
}

// ================ Delete password reset ================

/// Delete password reset request
#[derive(Debug, Clone)]
pub struct DeletePasswordResetDtoRequest {
    pub user_id: UserId,
}

/// Delete password reset response
#[derive(Debug, Clone)]
pub struct DeletePasswordResetDtoResponse();
