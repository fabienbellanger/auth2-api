//! Scope model

use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::id::Id;
use crate::domain::{use_cases::scope::ScopeUseCaseResponse, value_objects::scope_id::ScopeId};
use crate::infrastructure::api::response::ApiError; // TODO: No infrastructure here!!
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;
use std::str::FromStr;

#[derive(Debug, Clone, FromRow)]
pub struct ScopeModel {
    pub id: String,
    pub application_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl TryFrom<ScopeModel> for ScopeUseCaseResponse {
    type Error = ApiError;

    fn try_from(value: ScopeModel) -> Result<Self, Self::Error> {
        let id = ScopeId::new(value.id.as_str())?;
        let created_at = UtcDateTime::new(DateTime::<Utc>::from_naive_utc_and_offset(value.created_at, Utc));
        let updated_at = UtcDateTime::new(DateTime::<Utc>::from_naive_utc_and_offset(value.updated_at, Utc));
        let deleted_at = value
            .deleted_at
            .map(|deleted_at| UtcDateTime::new(DateTime::<Utc>::from_naive_utc_and_offset(deleted_at, Utc)));

        Ok(Self {
            id,
            application_id: Id::from_str(&value.application_id)?,
            created_at,
            updated_at,
            deleted_at,
        })
    }
}
