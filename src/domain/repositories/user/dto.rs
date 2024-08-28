//! DTO for user repository

use crate::domain::use_cases::user::create_user::{CreateUserUseCaseRequest, CreateUserUseCaseResponse};

/// Create user request
#[derive(Debug)]
pub struct CreateUserDtoRequest(pub CreateUserUseCaseRequest);

/// Create user response
#[derive(Debug)]
pub struct CreateUserDtoResponse(pub CreateUserUseCaseResponse);
