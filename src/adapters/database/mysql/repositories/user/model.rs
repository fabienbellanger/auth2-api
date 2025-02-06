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
            .map_err(|_| UserModelError::InvalidId(format!("Invalid user ID: `{}`", value.id)))?;
        let email = Email::new(&value.email)
            .map_err(|_| UserModelError::InvalidEmail(format!("Invalid user email: `{}`", value.email)))?;
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::entities::user::UserId;

    #[test]
    fn test_try_from_user_model_for_use_case_response() {
        let id = uuid::Uuid::new_v4().to_string();
        let model = UserModel {
            id: id.to_string(),
            lastname: "lastname".to_string(),
            firstname: "firstname".to_string(),
            email: "test@test.com".to_string(),
            created_at: DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            updated_at: DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            deleted_at: None,
        };
        let expected = UserUseCaseResponse {
            id: UserId::from_str(id.to_string().as_str()).unwrap(),
            lastname: "lastname".to_string(),
            firstname: "firstname".to_string(),
            email: Email::new("test@test.com").unwrap(),
            created_at: UtcDateTime::new(DateTime::from_timestamp(0, 0).unwrap()),
            updated_at: UtcDateTime::new(DateTime::from_timestamp(0, 0).unwrap()),
            deleted_at: None,
        };

        assert_eq!(UserUseCaseResponse::try_from(model).unwrap(), expected);
    }
}
