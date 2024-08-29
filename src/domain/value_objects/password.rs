//! Password value object representation
// TODO: Use bcrypt instead of sha512

use sha2::{Digest, Sha512};
use std::fmt::{Display, Formatter};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum PasswordError {
    #[error("Invalid password")]
    Invalid(#[from] validator::ValidationErrors),

    #[error("Password already hashed")]
    AlreadyHashed,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Validate)]
pub struct Password {
    #[validate(length(min = 8))]
    value: String,
    hashed: bool,
}

impl Password {
    /// Create and validate a new password
    ///
    /// # Example
    /// ```rust
    /// use fake::Fake;
    /// use fake::faker::internet::fr_fr::Password as FakePassword;
    /// use auth2_api::domain::value_objects::password::Password;
    ///
    /// let valid_password: String = FakePassword(8..12).fake();
    /// let password = Password::new(&valid_password, false).unwrap();
    /// assert_eq!(password.value(), valid_password);
    ///
    /// // `Password` implements the `Display` trait
    /// println!("{password}");
    ///
    /// assert!(Password::new("", false).is_err());
    /// let invalid_password: String = FakePassword(2..7).fake();
    /// assert!(Password::new(&invalid_password, false).is_err());
    /// assert!(Password::new(&invalid_password, true).is_ok());
    /// ```
    pub fn new(value: &str, hashed: bool) -> Result<Self, PasswordError> {
        let password = Self {
            value: value.to_string(),
            hashed,
        };

        if !password.hashed {
            password.validate()?;
        }

        Ok(password)
    }

    /// Get password value
    pub fn value(&self) -> String {
        self.value.clone()
    }

    /// Check if password is hashed
    pub fn hashed(&self) -> bool {
        self.hashed
    }

    /// Hash password and return  new hashed password
    ///
    /// # Example
    /// ```rust
    /// use auth2_api::domain::value_objects::password::{Password, PasswordError};
    ///
    /// let password = Password::new("1234567890", false).unwrap();
    /// let hashed_password = password.hash();
    /// assert!(hashed_password.is_ok());
    /// assert!(hashed_password.unwrap().hashed());
    ///
    /// let password = Password::new("1234567890", true).unwrap();
    /// let hashed_password = password.hash();
    /// assert!(hashed_password.is_err());
    /// if let Err(e) = hashed_password {
    ///     assert_eq!(e, PasswordError::AlreadyHashed);
    /// }
    /// ```
    pub fn hash(&self) -> Result<Self, PasswordError> {
        if self.hashed {
            return Err(PasswordError::AlreadyHashed);
        }

        Self::new(format!("{:x}", Sha512::digest(self.value.as_bytes())).as_ref(), true)
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
