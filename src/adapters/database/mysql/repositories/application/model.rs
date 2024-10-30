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
