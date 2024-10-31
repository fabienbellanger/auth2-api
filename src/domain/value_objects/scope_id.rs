//! Scope ID value object

use regex::Regex;
use std::{
    fmt::{Display, Formatter},
    sync::LazyLock,
};
use thiserror::Error;

/// Scope ID regex validation
/// Only lowercase letters, numbers, and `:.-` are allowed and must have at least 4 characters
static SCOPE_ID_REGEX: LazyLock<Result<Regex, ScopeIdError>> =
    LazyLock::new(|| Regex::new(r"^[a-z:.-]{4,}$").map_err(|_| ScopeIdError::RegexError()));

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ScopeIdError {
    #[error("Invalid scope ID regex")]
    RegexError(),

    #[error("{0}")]
    Invalid(String),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ScopeId {
    value: String,
}

impl ScopeId {
    /// Create and validate a new scope ID
    ///
    /// # Example
    /// ```rust
    /// use auth2_api::domain::value_objects::scope_id::ScopeId;
    ///
    /// let scope_id = ScopeId::new("test.lorem-ipsum:read");
    /// assert!(scope_id.is_ok());
    ///
    /// let invalid_scope_id = ScopeId::new("lor");
    /// assert!(invalid_scope_id.is_err());
    ///
    /// let invalid_scope_id = ScopeId::new("lorem.ipsum@read");
    /// assert!(invalid_scope_id.is_err());
    ///
    /// let invalid_scope_id = ScopeId::new("Lorem.ipsum:read");
    /// assert!(invalid_scope_id.is_err());
    /// ```
    pub fn new(value: &str) -> Result<Self, ScopeIdError> {
        let id = Self {
            value: value.to_string(),
        };

        if !SCOPE_ID_REGEX.clone()?.is_match(&id.value) {
            return Err(ScopeIdError::Invalid(
                "Invalid scope ID (`a-z:.-` with at least 4 characters)".to_string(),
            ));
        }

        Ok(id)
    }

    /// Get scope ID value
    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Display for ScopeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
