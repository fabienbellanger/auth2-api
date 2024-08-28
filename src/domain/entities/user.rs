//! User entity

use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::id::Id;
use crate::domain::value_objects::password::Password;

pub type UserId = Id;

/// User entity
#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password: Password,
    pub lastname: String,
    pub firstname: String,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
    pub deleted_at: Option<UtcDateTime>,
}

impl User {
    /// Return user full name
    ///
    /// ```
    /// use auth2_api::domain::entities::user::User;
    /// use auth2_api::domain::value_objects::email::Email;
    /// use auth2_api::domain::value_objects::id::Id;
    /// use auth2_api::domain::value_objects::password::Password;
    /// use uuid::Uuid;
    /// use auth2_api::domain::value_objects::datetime::UtcDateTime;
    ///
    /// let mut user = User {
    ///     id: Id::new().unwrap(),
    ///     lastname: "Doe".to_owned(),
    ///     firstname: "John".to_owned(),
    ///     email: Email::new("john.doe@test.com").unwrap(),
    ///     password: Password::new("1234567890", false).unwrap(),
    ///     created_at: UtcDateTime::now(),
    ///     updated_at: UtcDateTime::now(),
    ///     deleted_at: None,
    /// };
    ///
    /// assert_eq!(user.fullname(), "John Doe".to_owned());
    ///
    /// user.lastname = "".to_owned();
    /// assert_eq!(user.fullname(), "John".to_owned());
    ///
    /// user.firstname = "".to_owned();
    /// assert_eq!(user.fullname(), "".to_owned());
    /// ```
    pub fn fullname(&self) -> String {
        format!("{} {}", self.firstname, self.lastname).trim().to_string()
    }
}
