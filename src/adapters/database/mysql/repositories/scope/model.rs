//! Scope model

use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::id::Id;
use crate::domain::{use_cases::scope::ScopeUseCaseResponse, value_objects::scope_id::ScopeId};
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ScopeModelError {
    #[error("{0}")]
    InvalidScopeId(String),

    #[error("{0}")]
    InvalidApplicationId(String),
}

#[derive(Debug, Clone, FromRow)]
pub struct ScopeModel {
    pub id: String,
    pub application_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl TryFrom<ScopeModel> for ScopeUseCaseResponse {
    type Error = ScopeModelError;

    fn try_from(value: ScopeModel) -> Result<Self, Self::Error> {
        let id = ScopeId::new(value.id.as_str())
            .map_err(|_| ScopeModelError::InvalidScopeId(format!("Invalid scope ID: `{}`", value.id)))?;
        let application_id = Id::from_str(&value.application_id).map_err(|err| {
            ScopeModelError::InvalidApplicationId(format!(
                "`{}` is not a valid application ID: {err}",
                value.application_id
            ))
        })?;
        let created_at = UtcDateTime::new(DateTime::<Utc>::from_naive_utc_and_offset(value.created_at, Utc));
        let updated_at = UtcDateTime::new(DateTime::<Utc>::from_naive_utc_and_offset(value.updated_at, Utc));
        let deleted_at = value
            .deleted_at
            .map(|deleted_at| UtcDateTime::new(DateTime::<Utc>::from_naive_utc_and_offset(deleted_at, Utc)));

        Ok(Self {
            id,
            application_id,
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
    fn test_try_from_scope_model_for_use_case_response() {
        let application_id = uuid::Uuid::new_v4().to_string();
        let model = ScopeModel {
            id: "user:read".to_string(),
            application_id: application_id.to_string(),
            created_at: DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            updated_at: DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            deleted_at: None,
        };
        let expected = ScopeUseCaseResponse {
            id: ScopeId::new("user:read").unwrap(),
            application_id: ApplicationId::from_str(application_id.to_string().as_str()).unwrap(),
            created_at: UtcDateTime::new(DateTime::from_timestamp(0, 0).unwrap()),
            updated_at: UtcDateTime::new(DateTime::from_timestamp(0, 0).unwrap()),
            deleted_at: None,
        };

        assert_eq!(ScopeUseCaseResponse::try_from(model).unwrap(), expected);
    }
}
