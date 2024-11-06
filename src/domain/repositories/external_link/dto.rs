//! DTO for external link repository

use crate::domain::use_cases::external_link::{
    create_external_link::CreateExternalLinkUseCaseRequest, ExternalLinkUseCaseResponse,
};

// ================ Create external link ================

#[derive(Debug, Clone)]
pub struct CreateExternalLinkDtoRequest(pub CreateExternalLinkUseCaseRequest);

#[derive(Debug, Clone)]
pub struct CreateExternalLinkDtoResponse(pub ExternalLinkUseCaseResponse);
/*
// ================ Get external link by ID ================

#[derive(Debug, Clone)]
pub struct GetExternalLinkByIdDtoRequest(pub GetExternalLinkByIdUseCaseRequest);

#[derive(Debug, Clone)]
pub struct GetExternalLinkByIdDtoResponse(pub ExternalLinkUseCaseResponse);

// ================ Get all external links ================

#[derive(Debug, Clone)]
pub struct GetExternalLinksDtoRequest(pub GetExternalLinksUseCaseRequest);

#[derive(Debug, Clone)]
pub struct GetExternalLinksDtoResponse(pub Vec<ExternalLinkUseCaseResponse>);

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

// ================ Count external links ================

#[derive(Debug, Clone)]
pub struct CountExternalLinksDtoRequest {
    pub deleted: bool,
}

#[derive(Debug, Clone)]
pub struct CountExternalLinksDtoResponse(pub i64);

// ================ Restore external link ================

#[derive(Debug, Clone)]
pub struct RestoreExternalLinkDtoRequest(pub RestoreExternalLinkUseCaseRequest);

#[derive(Debug, Clone)]
pub struct RestoreExternalLinkDtoResponse(pub RestoreExternalLinkUseCaseResponse);
*/
