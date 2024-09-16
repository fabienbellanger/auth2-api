//! Password value object representation
//!
//! Attention: argon2 is slow!

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use passwords::{analyzer, scorer};
use std::fmt::{Display, Formatter};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum PasswordError {
    #[error("Invalid password: {0}")]
    Invalid(#[from] validator::ValidationErrors),

    #[error("Password hash error: {0}")]
    HashError(String),

    #[error("Password is not strong enough")]
    PasswordNotStrong(),
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Validate)]
pub struct Password {
    #[validate(length(min = 8))]
    value: String,
    original: Option<String>,
}

impl Password {
    /// Create and validate a new hashed password
    ///
    /// Password score:
    /// - 0 ~ 20 is very dangerous
    /// - 20 ~ 40 is dangerous
    /// - 40 ~ 60 is very weak
    /// - 60 ~ 80 is weak
    /// - 80 ~ 90 is good
    /// - 90 ~ 95 is strong
    /// - 95 ~ 99 is very strong
    /// - 99 ~ 100 is invulnerable
    ///
    /// # Example
    /// ```rust
    /// use fake::Fake;
    /// use fake::faker::internet::fr_fr::Password as FakePassword;
    /// use auth2_api::domain::value_objects::password::Password;
    ///
    /// let valid_password: String = FakePassword(16..25).fake();
    /// let password = Password::new(&valid_password, false);
    /// assert!(password.is_ok());
    ///
    /// // `Password` implements the `Display` trait
    /// println!("{}", password.unwrap());
    ///
    /// // Password not enough strong
    /// let not_strong_password: String = FakePassword(8..9).fake();
    /// let password = Password::new(&not_strong_password, false);
    /// assert!(password.is_err());
    ///
    /// assert!(Password::new("", false).is_err());
    /// let invalid_password: String = FakePassword(2..7).fake();
    /// assert!(Password::new(&invalid_password, false).is_err());
    /// ```
    pub fn new(value: &str, hashed: bool) -> Result<Self, PasswordError> {
        // Is password strong enough?
        if !hashed && scorer::score(&analyzer::analyze(value)) < 80.0 {
            return Err(PasswordError::PasswordNotStrong());
        }

        let mut password = Self {
            value: value.to_string(),
            original: None,
        };

        if !hashed {
            password.validate()?;

            password.original = Some(value.to_string());

            // Hash password
            let argon2 = Argon2::new(
                Algorithm::Argon2id,
                Version::default(),
                Params::new(512, 2, 1, None).map_err(|err| PasswordError::HashError(err.to_string()))?,
            );
            let salt = SaltString::generate(&mut OsRng);

            password.value = argon2
                .hash_password(value.as_bytes(), &salt)
                .map_err(|err| PasswordError::HashError(err.to_string()))?
                .to_string();
        }

        Ok(password)
    }

    /// Get password value
    pub fn value(&self) -> String {
        self.value.clone()
    }

    /// Get original password
    pub fn original(&self) -> Option<String> {
        self.original.clone()
    }

    /// Verify password
    ///
    /// # Example
    /// ```rust
    /// use auth2_api::domain::value_objects::password::{Password, PasswordError};
    ///
    /// // Valid password
    /// let hashed_password = Password::new("1A,R;(9h0Y&gYH5=7eY!ff", false).unwrap();
    /// assert!(hashed_password.verify("1A,R;(9h0Y&gYH5=7eY!ff").is_ok());
    ///
    /// // Invalid password
    /// assert!(hashed_password.verify("1234567").is_err());
    /// ```
    pub fn verify(&self, password: &str) -> Result<(), PasswordError> {
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::default(),
            Params::new(512, 2, 1, None).map_err(|err| PasswordError::HashError(err.to_string()))?,
        );
        let parsed_hash = PasswordHash::new(&self.value).map_err(|err| PasswordError::HashError(err.to_string()))?;

        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|err| PasswordError::HashError(err.to_string()))
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
