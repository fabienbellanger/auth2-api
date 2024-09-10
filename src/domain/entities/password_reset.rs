//! Password Reset entity

use crate::domain::entities::user::UserId;
use crate::domain::value_objects::datetime::UtcDateTime;
use chrono::Duration;
use std::ops::Add;
use uuid::Uuid;

/// Password reset token value
pub type PasswordResetTokenValue = String;

#[derive(Debug, Clone)]
pub struct PasswordReset {
    pub user_id: UserId,
    pub token: PasswordResetTokenValue,
    pub expired_at: UtcDateTime,
}

impl PasswordReset {
    /// Create a new password recovery
    pub fn new(user_id: UserId, expiration_duration: i64) -> Self {
        let now = UtcDateTime::now();
        let expired_at = match Duration::try_hours(expiration_duration) {
            Some(duration) => UtcDateTime::new(now.value().add(duration)),
            None => now,
        };

        Self {
            user_id,
            token: Uuid::new_v4().to_string(),
            expired_at,
        }
    }
}
