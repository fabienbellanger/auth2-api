//! DTO for external link repository

use crate::domain::use_cases::external_link::{
    create_external_link::CreateExternalLinkUseCaseRequest,
    delete_external_link::{DeleteExternalLinkUseCaseRequest, DeleteExternalLinkUseCaseResponse},
    get_external_link::GetExternalLinkByIdUseCaseRequest,
    get_external_links::GetExternalLinksUseCaseRequest,
    restore_external_link::{RestoreExternalLinkUseCaseRequest, RestoreExternalLinkUseCaseResponse},
    update_external_link::{UpdateExternalLinkUseCaseRequest, UpdateExternalLinkUseCaseResponse},
    ExternalLinkUseCaseResponse,
};

// ================ Create external link ================

#[derive(Debug, Clone)]
pub struct CreateExternalLinkDtoRequest(pub CreateExternalLinkUseCaseRequest);

#[derive(Debug, Clone)]
pub struct CreateExternalLinkDtoResponse(pub ExternalLinkUseCaseResponse);

// ================ Count external links ================

#[derive(Debug, Clone)]
pub struct CountExternalLinksDtoRequest {
    pub deleted: bool,
}

#[derive(Debug, Clone)]
pub struct CountExternalLinksDtoResponse(pub i64);

// ================ Get all external links ================

#[derive(Debug, Clone)]
pub struct GetExternalLinksDtoRequest(pub GetExternalLinksUseCaseRequest);

#[derive(Debug, Clone)]
pub struct GetExternalLinksDtoResponse(pub Vec<ExternalLinkUseCaseResponse>);

// ================ Get external link by ID ================

#[derive(Debug, Clone)]
pub struct GetExternalLinkByIdDtoRequest(pub GetExternalLinkByIdUseCaseRequest);

#[derive(Debug, Clone)]
pub struct GetExternalLinkByIdDtoResponse(pub ExternalLinkUseCaseResponse);

// ================ Update external link ================

#[derive(Debug, Clone)]
pub struct UpdateExternalLinkDtoRequest(pub UpdateExternalLinkUseCaseRequest);

#[derive(Debug, Clone)]
pub struct UpdateExternalLinkDtoResponse(pub UpdateExternalLinkUseCaseResponse);

// ================ Delete external link ================

#[derive(Debug, Clone)]
pub struct DeleteExternalLinkDtoRequest(pub DeleteExternalLinkUseCaseRequest);

#[derive(Debug, Clone)]
pub struct DeleteExternalLinkDtoResponse(pub DeleteExternalLinkUseCaseResponse);

// ================ Restore external link ================

#[derive(Debug, Clone)]
pub struct RestoreExternalLinkDtoRequest(pub RestoreExternalLinkUseCaseRequest);

#[derive(Debug, Clone)]
pub struct RestoreExternalLinkDtoResponse(pub RestoreExternalLinkUseCaseResponse);
