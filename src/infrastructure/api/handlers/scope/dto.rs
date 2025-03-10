//! Scopes handlers DTO

use crate::domain::use_cases::scope::ScopeUseCaseResponse;
use crate::domain::use_cases::scope::delete_scope::DeleteScopeUseCaseResponse;
use crate::domain::use_cases::scope::get_scopes::GetScopesUseCaseResponse;
use crate::domain::use_cases::scope::restore_scope::RestoreScopeUseCaseResponse;
use crate::infrastructure::api::handlers::filter::FilterRequest;
use serde::{Deserialize, Serialize};

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
}

// ================ Get scopes ================

pub type GetScopesFilterRequest = FilterRequest;

#[derive(Debug, Clone, Deserialize)]
pub struct GetScopesCustomFilterRequest {
    pub application_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetScopesResponse {
    pub total: i64,
    pub data: Vec<ScopeResponse>,
}

impl From<GetScopesUseCaseResponse> for GetScopesResponse {
    fn from(value: GetScopesUseCaseResponse) -> Self {
        Self {
            total: value.total,
            data: value.scopes.into_iter().map(|app| app.into()).collect(),
        }
    }
}

// ================ Delete scope ================

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DeleteScopeResponse();

impl From<DeleteScopeUseCaseResponse> for DeleteScopeResponse {
    fn from(_: DeleteScopeUseCaseResponse) -> Self {
        Self {}
    }
}

// ================ Restore scope ================

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RestoreScopeResponse();

impl From<RestoreScopeUseCaseResponse> for RestoreScopeResponse {
    fn from(_: RestoreScopeUseCaseResponse) -> Self {
        Self {}
    }
}
