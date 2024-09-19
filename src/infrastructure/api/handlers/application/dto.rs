//! Applications handlers DTO

use crate::domain::use_cases::application::create_application::CreateApplicationUseCaseRequest;
use crate::domain::use_cases::application::delete_application::DeleteApplicationUseCaseResponse;
use crate::domain::use_cases::application::get_applications::{
    GetApplicationsUseCaseRequest, GetApplicationsUseCaseResponse,
};
use crate::domain::use_cases::application::update_application::UpdateApplicationUseCaseResponse;
use crate::domain::use_cases::application::{ApplicationUseCaseError, ApplicationUseCaseResponse};
use crate::domain::utils::query_sort::{Filter, Sorts};
use crate::domain::value_objects::pagination::Pagination;
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

// ================ Get applications ================

/// Get applications request
// TODO: Duplicated code!
#[derive(Debug, Clone, Deserialize)]
pub struct GetApplicationsRequest {
    #[serde(rename(deserialize = "p"))]
    pub page: Option<u32>,

    #[serde(rename(deserialize = "l"))]
    pub limit: Option<u32>,

    #[serde(rename(deserialize = "s"))]
    pub sort: Option<String>,
}

impl TryFrom<GetApplicationsRequest> for GetApplicationsUseCaseRequest {
    type Error = ApplicationUseCaseError;

    fn try_from(value: GetApplicationsRequest) -> Result<Self, Self::Error> {
        let filter = match (value.page, value.limit, value.sort) {
            (Some(page), Some(limit), sort) => {
                let pagination = Some(Pagination::new(page, limit));
                let sorts = sort.map(|sort| Sorts::from(sort.as_str()));

                Some(Filter { pagination, sorts })
            }
            (_, _, Some(sort)) => Some(Filter {
                pagination: None,
                sorts: Some(Sorts::from(sort.as_str())),
            }),
            _ => None,
        };

        Ok(Self { filter })
    }
}

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
