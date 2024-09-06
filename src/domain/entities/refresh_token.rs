//! Refresh token entity

use crate::domain::entities::access_token::AccessToken;
use crate::domain::entities::user::UserId;
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::id::Id;
use chrono::Duration;
use std::ops::Add;
use thiserror::Error;

pub type RefreshTokenId = Id;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum RefreshTokenError {
    #[error("Invalid refresh token Id: {0}")]
    InvalidId(String),

    #[error("Invalid expiration duration: {0}")]
    InvalidExpirationDuration(String),
}

/// Refresh Token
#[derive(Debug, Clone, PartialEq)]
pub struct RefreshToken {
    pub refresh_token: RefreshTokenId,
    pub user_id: UserId,
    pub access_token: AccessToken,
    pub expired_at: UtcDateTime,
}

impl RefreshToken {
    /// Create a new refresh token
    ///
    /// # Example
    /// ```
    /// use auth2_api::domain::entities::access_token::AccessToken;
    /// use auth2_api::domain::entities::refresh_token::{RefreshToken, RefreshTokenError};
    /// use auth2_api::domain::value_objects::datetime::UtcDateTime;
    /// use auth2_api::domain::value_objects::id::Id;
    ///
    /// let access_token = AccessToken::new("token".to_owned(), UtcDateTime::now());
    /// let refresh_token = RefreshToken::create(Id::new().unwrap(), access_token.clone(), 7);
    /// assert!(refresh_token.is_ok());
    ///
    /// let refresh_token = RefreshToken::create(Id::new().unwrap(), access_token, 0);
    /// assert_eq!(refresh_token, Err(RefreshTokenError::InvalidExpirationDuration("0".to_string())));
    /// ```
    pub fn create(
        user_id: UserId,
        access_token: AccessToken,
        expiration_duration: i64,
    ) -> Result<Self, RefreshTokenError> {
        if expiration_duration <= 0 {
            return Err(RefreshTokenError::InvalidExpirationDuration(
                expiration_duration.to_string(),
            ));
        }

        let now = UtcDateTime::now();
        let expired_at = match Duration::try_days(expiration_duration) {
            Some(duration) => UtcDateTime::new(now.value().add(duration)),
            None => Err(RefreshTokenError::InvalidExpirationDuration(
                expiration_duration.to_string(),
            ))?,
        };

        Ok(Self {
            refresh_token: Id::new().map_err(|e| RefreshTokenError::InvalidId(e.to_string()))?,
            user_id,
            access_token,
            expired_at,
        })
    }

    /// Check if the token is valid (now <= expired datetime)
    ///
    /// # Example
    /// ```
    /// use auth2_api::domain::entities::access_token::AccessToken;
    /// use auth2_api::domain::entities::refresh_token::RefreshToken;
    /// use auth2_api::domain::value_objects::datetime::UtcDateTime;
    /// use auth2_api::domain::value_objects::id::Id;
    ///
    /// let access_token = AccessToken::new("token".to_owned(), UtcDateTime::now());
    /// let refresh_token = RefreshToken::create(Id::new().unwrap(), access_token.clone(), 7).unwrap();
    /// assert!(refresh_token.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        let now = UtcDateTime::now();

        now <= self.expired_at
    }
}
