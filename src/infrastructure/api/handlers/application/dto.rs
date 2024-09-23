//! Applications handlers DTO

use crate::domain::use_cases::application::create_application::CreateApplicationUseCaseRequest;
use crate::domain::use_cases::application::delete_application::DeleteApplicationUseCaseResponse;
use crate::domain::use_cases::application::get_applications::GetApplicationsUseCaseResponse;
use crate::domain::use_cases::application::update_application::UpdateApplicationUseCaseResponse;
use crate::domain::use_cases::application::ApplicationUseCaseResponse;
use crate::infrastructure::api::handlers::filter::FilterRequest;
use serde::{Deserialize, Serialize};

/// Application response
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ApplicationResponse {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl From<ApplicationUseCaseResponse> for ApplicationResponse {
    fn from(value: ApplicationUseCaseResponse) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
            deleted_at: value.deleted_at.map(|dt| dt.to_string()),
        }
    }
}

// ================ Application creation ================

/// Create application request
#[derive(Debug, Clone, Deserialize)]
pub struct CreateApplicationRequest {
    pub name: String,
}

impl From<CreateApplicationRequest> for CreateApplicationUseCaseRequest {
    fn from(value: CreateApplicationRequest) -> Self {
        Self { name: value.name }
    }
}

// ================ Get applications ================

/// Get applications request
pub type GetApplicationsRequest = FilterRequest;

/// Get applications response
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetApplicationsResponse {
    pub total: i64,
    pub data: Vec<ApplicationResponse>,
}

impl From<GetApplicationsUseCaseResponse> for GetApplicationsResponse {
    fn from(value: GetApplicationsUseCaseResponse) -> Self {
        Self {
            total: value.total,
            data: value.applications.into_iter().map(|app| app.into()).collect(),
        }
    }
}

// ================ Delete application ================

/// Delete an application response
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApplicationResponse();

impl From<DeleteApplicationUseCaseResponse> for DeleteApplicationResponse {
    fn from(_: DeleteApplicationUseCaseResponse) -> Self {
        Self {}
    }
}

// ================ Update application ================

/// Update an application request
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateApplicationRequest {
    pub name: String,
}

/// Update an application response
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApplicationResponse();

impl From<UpdateApplicationUseCaseResponse> for UpdateApplicationResponse {
    fn from(_: UpdateApplicationUseCaseResponse) -> Self {
        Self {}
    }
}
