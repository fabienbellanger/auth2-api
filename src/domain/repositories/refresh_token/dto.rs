//! DTO for refresh token repository

use crate::domain::entities::access_token::AccessToken;
use crate::domain::entities::refresh_token::{RefreshToken, RefreshTokenId};
use crate::domain::entities::user::UserId;

// ================ Refresh token creation ================

#[derive(Debug, Clone)]
pub struct CreateRefreshTokenDtoRequest {
    pub refresh_token: RefreshToken,
    pub access_token: AccessToken,
    pub user_id: UserId,
}

#[derive(Debug, Clone)]
pub struct CreateRefreshTokenDtoResponse();

// ================ Get refresh token ================

#[derive(Debug, Clone)]
pub struct GetRefreshTokenDtoRequest(pub RefreshTokenId);

#[derive(Debug, Clone)]
pub struct GetRefreshTokenDtoResponse {
    pub user_id: UserId,
}

// ================ Delete refresh token ================

#[derive(Debug, Clone)]
pub struct DeleteRefreshTokenDtoRequest(pub RefreshTokenId);

#[derive(Debug, Clone)]
pub struct DeleteRefreshTokenDtoResponse();

// ================ Delete expired refresh tokens ================

#[derive(Debug, Clone)]
pub struct DeleteExpiredRefreshTokensDtoRequest();

#[derive(Debug, Clone)]
pub struct DeleteExpiredRefreshTokensDtoResponse {
    pub deleted: u64,
}
