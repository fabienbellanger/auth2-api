//! Scopes handlers DTO

use crate::domain::use_cases::scope::create_scope::CreateScopeUseCaseRequest;
use crate::domain::use_cases::scope::{ScopeUseCaseError, ScopeUseCaseResponse};
use crate::domain::value_objects::id::Id;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Scope response
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ScopeResponse {
    pub id: String,
    pub application_id: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl From<ScopeUseCaseResponse> for ScopeResponse {
    fn from(value: ScopeUseCaseResponse) -> Self {
        Self {
            id: value.id.to_string(),
            application_id: value.application_id.to_string(),
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
            deleted_at: value.deleted_at.map(|dt| dt.to_string()),
        }
    }
}

// ================ Scope creation ================

#[derive(Debug, Clone, Deserialize)]
pub struct CreateScopeRequest {
    pub id: String,
    pub application_id: String,
}

impl TryFrom<CreateScopeRequest> for CreateScopeUseCaseRequest {
    type Error = ScopeUseCaseError;

    fn try_from(value: CreateScopeRequest) -> Result<Self, Self::Error> {
        Ok(CreateScopeUseCaseRequest {
            id: value.id.to_string(),
            application_id: Id::from_str(&value.application_id)
                .map_err(|err| Self::Error::InvalidApplicationId(err.to_string()))?,
        })
    }
}
