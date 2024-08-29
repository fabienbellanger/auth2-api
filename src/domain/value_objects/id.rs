//! Id value object representation

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum IdError {
    #[error("Invalid UUID id: {0}")]
    Invalid(String),
}

impl From<uuid::Error> for IdError {
    fn from(error: uuid::Error) -> Self {
        IdError::Invalid(error.to_string())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Id {
    value: Uuid,
}

impl Id {
    /// Create and validate a new id
    ///
    /// # Example
    /// ```rust
    /// use auth2_api::domain::value_objects::id::Id;
    ///
    /// let id = Id::new();
    /// assert!(id.is_ok());
    /// ```
    pub fn new() -> Result<Self, IdError> {
        Ok(Self { value: Uuid::new_v4() })
    }

    /// Get email value
    pub fn value(&self) -> Uuid {
        self.value
    }
}

impl FromStr for Id {
    type Err = IdError;

    /// Create and validate a new id from a string
    ///
    /// # Example
    /// ```rust
    /// use std::str::FromStr;
    /// use auth2_api::domain::value_objects::id::Id;
    ///
    /// let id = Id::from_str("");
    /// assert!(id.is_err());
    ///
    /// let id = Id::from_str("550e8400-e29b-41d4-a716-446655440000");
    /// assert!(id.is_ok());
    /// assert_eq!(id.unwrap().to_string(), "550e8400-e29b-41d4-a716-446655440000".to_owned());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Self {
            value: Uuid::parse_str(s)?,
        };

        Ok(id)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
