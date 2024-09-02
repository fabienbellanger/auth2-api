//! Email value object representation

use std::fmt::{Display, Formatter};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum EmailError {
    #[error("Invalid email: {0}")]
    Invalid(#[from] validator::ValidationErrors),
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Validate)]
pub struct Email {
    #[validate(email)]
    value: String,
}

impl Email {
    /// Create and validate a new email
    ///
    /// # Example
    /// ```rust
    /// use auth2_api::domain::value_objects::email::Email;
    ///
    /// let email = Email::new("lorem");
    /// assert!(email.is_err());
    ///
    /// let email = Email::new("test@gmail.com");
    /// assert!(email.is_ok());
    /// assert_eq!(email.unwrap().value(), "test@gmail.com".to_string());
    /// ```
    pub fn new(value: &str) -> Result<Self, EmailError> {
        let email = Self {
            value: value.to_string(),
        };

        email.validate()?;

        Ok(email)
    }

    /// Get email value
    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
