//! Password value object representation

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use std::fmt::{Display, Formatter};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum PasswordError {
    #[error("Invalid password")]
    Invalid(#[from] validator::ValidationErrors),

    #[error("Password hash error: {0}")]
    HashError(String),
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Validate)]
pub struct Password {
    #[validate(length(min = 8))]
    value: String,
}

impl Password {
    /// Create and validate a new hashed password
    ///
    /// # Example
    /// ```rust
    /// use fake::Fake;
    /// use fake::faker::internet::fr_fr::Password as FakePassword;
    /// use auth2_api::domain::value_objects::password::Password;
    ///
    /// let valid_password: String = FakePassword(8..12).fake();
    /// let password = Password::new(&valid_password, false);
    /// assert!(password.is_ok());
    ///
    /// // `Password` implements the `Display` trait
    /// println!("{}", password.unwrap());
    ///
    /// assert!(Password::new("", false).is_err());
    /// let invalid_password: String = FakePassword(2..7).fake();
    /// assert!(Password::new(&invalid_password, false).is_err());
    /// ```
    pub fn new(value: &str, hashed: bool) -> Result<Self, PasswordError> {
        let mut password = Self {
            value: value.to_string(),
        };

        if !hashed {
            password.validate()?;

            // Hash password
            let argon2 = Argon2::default();
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

    /// Verify password
    ///
    /// # Example
    /// ```rust
    /// use auth2_api::domain::value_objects::password::{Password, PasswordError};
    ///
    /// // Valid password
    /// let hashed_password = Password::new("1234567890", false).unwrap();
    /// dbg!(&hashed_password);
    /// assert!(hashed_password.verify("1234567890").is_ok());
    ///
    /// // Invalid password
    /// assert!(hashed_password.verify("1234567").is_err());
    /// ```
    pub fn verify(&self, password: &str) -> Result<(), PasswordError> {
        let argon2 = Argon2::default();
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
