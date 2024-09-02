//! JWT payload

use crate::domain::services::security::jwt::Jwt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Payload errors
#[derive(Debug, Clone, PartialEq, Error)]
pub enum PayloadError {
    #[error("Missing token")]
    MissingToken,

    #[error("Invalid token: {0}")]
    ParseTokenError(String),

    #[error("Invalid headers")]
    InvalidHeaders,
}

/// JWT payload
// TODO: Missing roles / scopes
#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    /// Subject: User ID
    pub sub: String,

    /// Application ID
    pub application_id: String, // TODO: Custom type?

    /// Client ID
    pub client_id: String, // TODO: Custom type?

    /// Expiration time
    pub exp: i64,

    /// Issued at
    pub iat: i64,

    /// Not before at
    pub nbf: i64,
}

pub trait PayloadExtractor<H> {
    /// Extract payload from request headers
    fn try_from_headers(headers: &H, jwt: &Jwt) -> Result<Payload, PayloadError>;
}

/// Data included in the payload
#[derive(Debug, Clone)]
pub struct PayloadData {
    pub user_id: String,        // TODO: Custom type?
    pub application_id: String, // TODO: Custom type?
    pub client_id: String,      // TODO: Custom type?
}

impl From<Payload> for PayloadData {
    fn from(payload: Payload) -> Self {
        Self {
            user_id: payload.sub,
            application_id: payload.application_id,
            client_id: payload.client_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload_data_from_payload() {
        let payload = Payload {
            sub: "user_id".to_owned(),
            application_id: "app_id".to_owned(),
            client_id: "client_id".to_owned(),
            exp: 0,
            iat: 0,
            nbf: 0,
        };

        let data = PayloadData::from(payload);

        assert_eq!(data.user_id, "user_id");
        assert_eq!(data.application_id, "app_id");
        assert_eq!(data.client_id, "client_id");
    }
}
