//! JWT

use crate::domain::entities::access_token::AccessToken;
use crate::domain::services::security::payload::{Payload, PayloadData};
use crate::domain::value_objects::datetime::UtcDateTime;
use chrono::Utc;
use jsonwebtoken::errors::ErrorKind::ExpiredSignature;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Validation};
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use thiserror::Error;

/// JWT errors
#[derive(Debug, Clone, PartialEq, Error)]
pub enum JwtError {
    #[error("Parse token error: {0}")]
    ParseError(String),

    #[error("Generate token error: {0}")]
    GenerateError(String),

    #[error("Invalid or unsupported algorithm: {0}")]
    InvalidAlgorithm(String),

    #[error("Encoding key error: {0}")]
    EncodingKeyError(String),

    #[error("Decoding key error: {0}")]
    DecodingKeyError(String),

    #[error("Expired token")]
    ExpiredToken,
}

/// JWT representation
#[derive(Clone)]
pub struct Jwt {
    /// The algorithm supported for signing/verifying JWT
    algorithm: Algorithm,

    /// Access Token lifetime (in minute)
    access_lifetime: i64,

    /// Refresh Token lifetime (in day)
    refresh_lifetime: i64,

    /// Encoding key
    encoding_key: Option<EncodingKey>,

    /// Decoding key
    decoding_key: Option<DecodingKey>,
}

impl Default for Jwt {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::HS512,
            access_lifetime: 15, // 15 minutes
            refresh_lifetime: 7, // 7 days
            encoding_key: None,
            decoding_key: None,
        }
    }
}

impl Debug for Jwt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JWT => algo: {:?}, access_lifetime: {}, refresh_lifetime: {}",
            self.algorithm, self.access_lifetime, self.refresh_lifetime
        )
    }
}

impl Jwt {
    /// Initialize a new `Jwt`
    pub fn init(
        algorithm: &str,
        access_lifetime: i64,
        refresh_lifetime: i64,
        secret: Option<&str>,
        private_key: Option<&str>,
        public_key: Option<&str>,
    ) -> Result<Self, JwtError> {
        let mut jwt = Jwt {
            algorithm: Self::algorithm_from_str(algorithm)?,
            access_lifetime,
            refresh_lifetime,
            ..Default::default()
        };

        // Encoding key
        match (secret, private_key, jwt.use_secret()) {
            (Some(secret), _, true) => jwt.set_encoding_key(secret.trim())?,
            (_, Some(private_key), false) => jwt.set_encoding_key(private_key.trim())?,
            _ => return Err(JwtError::EncodingKeyError("invalid JWT encoding key".to_owned())),
        }

        // Decoding key
        match (secret, public_key, jwt.use_secret()) {
            (Some(secret), _, true) => jwt.set_decoding_key(secret.trim())?,
            (_, Some(public_key), false) => jwt.set_decoding_key(public_key.trim())?,
            _ => return Err(JwtError::DecodingKeyError("invalid JWT decoding key".to_owned())),
        }

        Ok(jwt)
    }

    /// Get refresh token lifetime
    pub fn refresh_lifetime(&self) -> i64 {
        self.refresh_lifetime
    }

    /// Update access token lifetime (in minute)
    pub fn set_access_lifetime(&mut self, duration: i64) {
        self.access_lifetime = duration;
    }

    /// Update refresh token lifetime (in day)
    pub fn set_refresh_lifetime(&mut self, duration: i64) {
        self.refresh_lifetime = duration;
    }

    /// Update encoding key
    pub fn set_encoding_key(&mut self, secret: &str) -> Result<(), JwtError> {
        let key = match self.algorithm {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => EncodingKey::from_secret(secret.as_bytes()),
            Algorithm::ES256 | Algorithm::ES384 => EncodingKey::from_ec_pem(secret.as_bytes())
                .map_err(|err| JwtError::EncodingKeyError(err.to_string()))?,
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => EncodingKey::from_rsa_pem(secret.as_bytes())
                .map_err(|err| JwtError::EncodingKeyError(err.to_string()))?,
            Algorithm::PS256 | Algorithm::PS384 | Algorithm::PS512 => EncodingKey::from_rsa_pem(secret.as_bytes())
                .map_err(|err| JwtError::EncodingKeyError(err.to_string()))?,
            Algorithm::EdDSA => EncodingKey::from_ed_pem(secret.as_bytes())
                .map_err(|err| JwtError::EncodingKeyError(err.to_string()))?,
        };

        self.encoding_key = Some(key);

        Ok(())
    }

    /// Update decoding key
    pub fn set_decoding_key(&mut self, secret: &str) -> Result<(), JwtError> {
        let key = match self.algorithm {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => DecodingKey::from_secret(secret.as_bytes()),
            Algorithm::ES256 | Algorithm::ES384 => DecodingKey::from_ec_pem(secret.as_bytes())
                .map_err(|err| JwtError::DecodingKeyError(err.to_string()))?,
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => DecodingKey::from_rsa_pem(secret.as_bytes())
                .map_err(|err| JwtError::DecodingKeyError(err.to_string()))?,
            Algorithm::PS256 | Algorithm::PS384 | Algorithm::PS512 => DecodingKey::from_rsa_pem(secret.as_bytes())
                .map_err(|err| JwtError::DecodingKeyError(err.to_string()))?,
            Algorithm::EdDSA => DecodingKey::from_ed_pem(secret.as_bytes())
                .map_err(|err| JwtError::DecodingKeyError(err.to_string()))?,
        };

        self.decoding_key = Some(key);

        Ok(())
    }

    /// Generate JWT
    pub fn generate(&self, data: PayloadData) -> Result<AccessToken, JwtError> {
        let header = jsonwebtoken::Header::new(self.algorithm);
        let now = Utc::now();
        let access_expired_at = now.add(chrono::Duration::minutes(self.access_lifetime));

        let payload = Payload {
            sub: data.user_id,
            exp: access_expired_at.timestamp(),
            iat: now.timestamp(),
            nbf: now.timestamp(),
            application_id: data.application_id,
            client_id: data.client_id,
        };

        match self.encoding_key.clone() {
            Some(encoding_key) => {
                let token = encode(&header, &payload, &encoding_key)
                    .map_err(|err| JwtError::EncodingKeyError(err.to_string()))?;

                Ok(AccessToken {
                    token,
                    expired_at: UtcDateTime::from(access_expired_at),
                })
            }
            _ => Err(JwtError::EncodingKeyError("empty key".to_owned())),
        }
    }

    /// Parse JWT
    pub fn parse(&self, token: &AccessToken) -> Result<Payload, JwtError> {
        let validation = Validation::new(self.algorithm);

        match self.decoding_key.clone() {
            Some(decoding_key) => {
                let token =
                    decode::<Payload>(&token.token, &decoding_key, &validation).map_err(|err| match err.kind() {
                        ExpiredSignature => JwtError::ExpiredToken,
                        _ => JwtError::DecodingKeyError(err.to_string()),
                    })?;

                Ok(token.claims)
            }
            _ => Err(JwtError::DecodingKeyError("empty key".to_owned())),
        }
    }

    /// Return true if a secret key is used instead of a pair of keys
    fn use_secret(&self) -> bool {
        self.algorithm == Algorithm::HS256 || self.algorithm == Algorithm::HS384 || self.algorithm == Algorithm::HS512
    }

    /// Convert `&str` to `Algorithm`
    fn algorithm_from_str(algo: &str) -> Result<Algorithm, JwtError> {
        Ok(match algo {
            "HS256" => Algorithm::HS256,
            "HS384" => Algorithm::HS384,
            "HS512" => Algorithm::HS512,
            "ES256" => Algorithm::ES256,
            "ES384" => Algorithm::ES384,
            _ => {
                return Err(JwtError::InvalidAlgorithm(algo.to_string()));
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_use_secret() {
        let jwt = Jwt::default();
        assert!(jwt.use_secret());

        let mut jwt = Jwt::default();
        jwt.algorithm = Algorithm::ES256;
        assert!(!jwt.use_secret());

        jwt.algorithm = Algorithm::HS256;
        assert!(jwt.use_secret());
    }

    #[test]
    fn test_jwt_algorithm_from_str() {
        assert_eq!(Jwt::algorithm_from_str("HS256").unwrap(), Algorithm::HS256);
        assert_eq!(Jwt::algorithm_from_str("HS384").unwrap(), Algorithm::HS384);
        assert_eq!(Jwt::algorithm_from_str("HS512").unwrap(), Algorithm::HS512);
        assert_eq!(Jwt::algorithm_from_str("ES256").unwrap(), Algorithm::ES256);
        assert_eq!(Jwt::algorithm_from_str("ES384").unwrap(), Algorithm::ES384);

        let invalid_algo = Jwt::algorithm_from_str("ES512");
        assert!(invalid_algo.is_err());
        if let Err(e) = invalid_algo {
            assert_eq!(e, JwtError::InvalidAlgorithm("ES512".to_string()));
        }
    }
}
