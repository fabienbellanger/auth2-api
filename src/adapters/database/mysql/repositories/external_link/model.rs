//! External link model

use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;
use std::str::FromStr;
use thiserror::Error;

use crate::domain::{
    entities::external_link::ExternalLinkId, use_cases::external_link::ExternalLinkUseCaseResponse,
    value_objects::datetime::UtcDateTime,
};

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ExternalLinkModelError {
    #[error("{0}")]
    InvalidId(String),
}

#[derive(Debug, Clone, FromRow)]
pub struct ExternalLinkModel {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl TryFrom<ExternalLinkModel> for ExternalLinkUseCaseResponse {
    type Error = ExternalLinkModelError;

    fn try_from(value: ExternalLinkModel) -> Result<Self, Self::Error> {
        let id = ExternalLinkId::from_str(&value.id)
            .map_err(|_| ExternalLinkModelError::InvalidId(format!("Invalid external link ID: `{}`", value.id)))?;
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

    #[test]
    fn test_try_from_external_link_model_for_use_case_response() {
        let id = uuid::Uuid::new_v4().to_string();
        let model = ExternalLinkModel {
            id: id.to_string(),
            name: "name".to_string(),
            created_at: DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            updated_at: DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            deleted_at: None,
        };
        let expected = ExternalLinkUseCaseResponse {
            id: ExternalLinkId::from_str(id.to_string().as_str()).unwrap(),
            name: "name".to_string(),
            created_at: UtcDateTime::new(DateTime::from_timestamp(0, 0).unwrap()),
            updated_at: UtcDateTime::new(DateTime::from_timestamp(0, 0).unwrap()),
            deleted_at: None,
        };

        assert_eq!(ExternalLinkUseCaseResponse::try_from(model).unwrap(), expected);
    }
}
