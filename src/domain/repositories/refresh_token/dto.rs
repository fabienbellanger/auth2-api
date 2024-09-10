//! DTO for refresh token repository

use crate::domain::entities::access_token::AccessToken;
use crate::domain::entities::refresh_token::{RefreshToken, RefreshTokenId};
use crate::domain::entities::user::UserId;

// ================ Refresh token creation ================

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

// ================ Get refresh token ================

/// Get refresh token request
#[derive(Debug, Clone)]
pub struct GetRefreshTokenDtoRequest(pub RefreshTokenId);

/// Get refresh token response
#[derive(Debug, Clone)]
pub struct GetRefreshTokenDtoResponse {
    pub user_id: UserId,
}

// ================ Delete refresh token ================

/// Delete refresh token request
#[derive(Debug, Clone)]
pub struct DeleteRefreshTokenDtoRequest(pub RefreshTokenId);

/// Delete refresh token response
#[derive(Debug, Clone)]
pub struct DeleteRefreshTokenDtoResponse();

// ================ Delete expired refresh tokens ================

#[derive(Debug, Clone)]
pub struct DeleteExpiredRefreshTokensDtoRequest();

#[derive(Debug, Clone)]
pub struct DeleteExpiredRefreshTokensDtoResponse {
    pub deleted: u64,
}
