//! Scope entity

use crate::domain::entities::application::Application;
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::scope_id::ScopeId;

/// Scope entity
#[derive(Debug, Clone)]
pub struct Scope {
    pub id: ScopeId,
    pub application: Application,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
    pub deleted_at: Option<UtcDateTime>,
}

impl Scope {
    /// Create a new scope
    pub fn new(id: ScopeId, application: &Application) -> Self {
        Self {
            id,
            application: application.clone(),
            created_at: UtcDateTime::now(),
            updated_at: UtcDateTime::now(),
            deleted_at: None,
        }
    }
}
