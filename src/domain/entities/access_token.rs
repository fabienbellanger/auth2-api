//! Access token entity

use crate::domain::value_objects::datetime::UtcDateTime;

/// Access Token
#[derive(Debug, Clone)]
pub struct AccessToken {
    /// Token
    pub token: String,

    /// Expiration time
    pub expired_at: UtcDateTime,
}

impl AccessToken {
    /// Create a new access token
    pub fn new(token: String, expired_at: UtcDateTime) -> Self {
        Self { token, expired_at }
    }
}
