//! Application model

use crate::domain::use_cases::application::ApplicationUseCaseResponse;
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::id::Id;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ApplicationModelError {
    #[error("{0}")]
    InvalidId(String),
}

#[derive(Debug, Clone, FromRow)]
pub struct ApplicationModel {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl TryFrom<ApplicationModel> for ApplicationUseCaseResponse {
    type Error = ApplicationModelError;

    fn try_from(value: ApplicationModel) -> Result<Self, Self::Error> {
        let id = Id::from_str(&value.id)
            .map_err(|_| ApplicationModelError::InvalidId(format!("Invalid application ID: `{}`", value.id)))?;
        let created_at = UtcDateTime::new(DateTime::<Utc>::from_naive_utc_and_offset(value.created_at, Utc));
        let updated_at = UtcDateTime::new(DateTime::<Utc>::from_naive_utc_and_offset(value.updated_at, Utc));
        let deleted_at = value
            .deleted_at
            .map(|deleted_at| UtcDateTime::new(DateTime::<Utc>::from_naive_utc_and_offset(deleted_at, Utc)));

        Ok(Self {
            id,
            name: value.name,
            created_at,
            updated_at,
            deleted_at,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::entities::application::ApplicationId;

    #[test]
    fn test_try_from_application_model_for_use_case_response() {
        let id = uuid::Uuid::new_v4().to_string();
        let model = ApplicationModel {
            id: id.to_string(),
            name: "name".to_string(),
            created_at: DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            updated_at: DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            deleted_at: None,
        };
        let expected = ApplicationUseCaseResponse {
            id: ApplicationId::from_str(id.to_string().as_str()).unwrap(),
            name: "name".to_string(),
            created_at: UtcDateTime::new(DateTime::from_timestamp(0, 0).unwrap()),
            updated_at: UtcDateTime::new(DateTime::from_timestamp(0, 0).unwrap()),
            deleted_at: None,
        };

        assert_eq!(ApplicationUseCaseResponse::try_from(model).unwrap(), expected);
    }
}
