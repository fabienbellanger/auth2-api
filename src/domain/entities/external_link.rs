//! ExternalLink entity represents a connection with another application

use crate::domain::value_objects::{datetime::UtcDateTime, id::Id};

pub type ExternalLinkId = Id;

/// ExternalLink entity
#[derive(Debug, Clone)]
pub struct ExternalLink {
    pub id: ExternalLinkId,
    pub name: String,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
    pub deleted_at: Option<UtcDateTime>,
}
