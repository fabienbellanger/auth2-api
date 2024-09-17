//! Applications handlers DTO

use crate::domain::use_cases::application::create_application::CreateApplicationUseCaseRequest;
use crate::domain::use_cases::application::ApplicationUseCaseResponse;
use serde::{Deserialize, Serialize};

/// Application response
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ApplicationResponse {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<ApplicationUseCaseResponse> for ApplicationResponse {
    fn from(value: ApplicationUseCaseResponse) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
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
