//! DTO for application repository

use crate::domain::use_cases::application::create_application::CreateApplicationUseCaseRequest;
use crate::domain::use_cases::application::ApplicationUseCaseResponse;

// ================ Create application ================
#[derive(Debug, Clone)]
pub struct CreateApplicationDtoRequest(pub CreateApplicationUseCaseRequest);

#[derive(Debug, Clone)]
pub struct CreateApplicationDtoResponse(pub ApplicationUseCaseResponse);
