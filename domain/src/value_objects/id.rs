//! ID value object representation

use auth2_api_shared::error::{ApiError, ApiResult};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Id {
    value: Uuid,
}

impl Id {
    /// Create and validate a new id
    ///
    /// # Example
    /// ```rust
    /// use auth2_api_domain::value_objects::id::Id;
    ///
    /// let id = Id::new().unwrap();
    /// ```
    pub fn new() -> ApiResult<Self> {
        Ok(Self { value: Uuid::new_v4() })
    }

    /// Get id value
    pub fn value(&self) -> Uuid {
        self.value.clone()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<&str> for Id {
    type Error = ApiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            value: Uuid::parse_str(value)?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_valid_str() {
        let uid = Uuid::new_v4();
        let id = Id::try_from(uid.to_string().as_str());

        assert!(id.is_ok());
        assert_eq!(uid, id.unwrap().value());
    }

    #[test]
    fn test_from_invalid_str() {
        let id = Id::try_from("45154");

        assert!(id.is_err());
    }
}
