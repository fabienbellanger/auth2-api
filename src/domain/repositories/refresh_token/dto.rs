//! DTO for refresh token repository

use crate::domain::entities::access_token::AccessToken;
use crate::domain::entities::refresh_token::RefreshToken;
use crate::domain::entities::user::UserId;

// ================ refresh token creation ================

/// Create refresh token request
#[derive(Debug, Clone)]
pub struct CreateRefreshTokenDtoRequest {
    pub refresh_token: RefreshToken,
    pub access_token: AccessToken,
    pub user_id: UserId,
}

/// Create refresh token response
#[derive(Debug, Clone)]
pub struct CreateRefreshTokenDtoResponse();
