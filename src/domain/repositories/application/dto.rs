//! DTO for application repository

use crate::domain::use_cases::application::ApplicationUseCaseResponse;
use crate::domain::use_cases::application::create_application::CreateApplicationUseCaseRequest;
use crate::domain::use_cases::application::delete_application::{
    DeleteApplicationUseCaseRequest, DeleteApplicationUseCaseResponse,
};
use crate::domain::use_cases::application::get_application::GetApplicationByIdUseCaseRequest;
use crate::domain::use_cases::application::get_applications::GetApplicationsUseCaseRequest;
use crate::domain::use_cases::application::restore_application::{
    RestoreApplicationUseCaseRequest, RestoreApplicationUseCaseResponse,
};
use crate::domain::use_cases::application::update_application::{
    UpdateApplicationUseCaseRequest, UpdateApplicationUseCaseResponse,
};

// ================ Create application ================

#[derive(Debug, Clone)]
pub struct CreateApplicationDtoRequest(pub CreateApplicationUseCaseRequest);

#[derive(Debug, Clone)]
pub struct CreateApplicationDtoResponse(pub ApplicationUseCaseResponse);

// ================ Get application by ID ================

#[derive(Debug, Clone)]
pub struct GetApplicationByIdDtoRequest(pub GetApplicationByIdUseCaseRequest);

#[derive(Debug, Clone)]
pub struct GetApplicationByIdDtoResponse(pub ApplicationUseCaseResponse);

// ================ Get all applications ================

#[derive(Debug, Clone)]
pub struct GetApplicationsDtoRequest(pub GetApplicationsUseCaseRequest);

#[derive(Debug, Clone)]
pub struct GetApplicationsDtoResponse(pub Vec<ApplicationUseCaseResponse>);

// ================ Update application ================

#[derive(Debug, Clone)]
pub struct UpdateApplicationDtoRequest(pub UpdateApplicationUseCaseRequest);

#[derive(Debug, Clone)]
pub struct UpdateApplicationDtoResponse(pub UpdateApplicationUseCaseResponse);

// ================ Delete application ================

#[derive(Debug, Clone)]
pub struct DeleteApplicationDtoRequest(pub DeleteApplicationUseCaseRequest);

#[derive(Debug, Clone)]
pub struct DeleteApplicationDtoResponse(pub DeleteApplicationUseCaseResponse);

// ================ Count applications ================

#[derive(Debug, Clone)]
pub struct CountApplicationsDtoRequest {
    pub deleted: bool,
}

#[derive(Debug, Clone)]
pub struct CountApplicationsDtoResponse(pub i64);

// ================ Restore application ================

#[derive(Debug, Clone)]
pub struct RestoreApplicationDtoRequest(pub RestoreApplicationUseCaseRequest);

#[derive(Debug, Clone)]
pub struct RestoreApplicationDtoResponse(pub RestoreApplicationUseCaseResponse);
