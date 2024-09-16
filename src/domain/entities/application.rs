//! Application entity

use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::id::Id;

pub type ApplicationId = Id;

/// Application entity
#[derive(Debug, Clone)]
pub struct Application {
    pub id: ApplicationId,
    pub name: String,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
    pub deleted_at: Option<UtcDateTime>,
}
