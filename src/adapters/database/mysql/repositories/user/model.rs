//! User model

use crate::domain::use_cases::user::UserUseCaseResponse;
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::id::Id;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum UserModelError {
    #[error("{0}")]
    InvalidId(String),

    #[error("{0}")]
    InvalidEmail(String),
}

#[derive(Debug, Clone, FromRow)]
pub struct UserModel {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl TryFrom<UserModel> for UserUseCaseResponse {
    type Error = UserModelError;

    fn try_from(value: UserModel) -> Result<Self, Self::Error> {
        let id = Id::from_str(&value.id)
            .map_err(|err| UserModelError::InvalidId(format!("`{}` is not a valid user ID: {err}", value.id)))?;
        let email = Email::new(&value.email)
            .map_err(|err| UserModelError::InvalidEmail(format!("`{}` is not a valid email: {err}", value.email)))?;
        let created_at = UtcDateTime::new(DateTime::<Utc>::from_naive_utc_and_offset(value.created_at, Utc));
        let updated_at = UtcDateTime::new(DateTime::<Utc>::from_naive_utc_and_offset(value.updated_at, Utc));
        let deleted_at = value
            .deleted_at
            .map(|dt| UtcDateTime::new(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)));

        Ok(Self {
            id,
            email,
            lastname: value.lastname,
            firstname: value.firstname,
            created_at,
            updated_at,
            deleted_at,
        })
    }
}
