//! DTO for scope repository

use crate::domain::use_cases::scope::create_scope::CreateScopeUseCaseRequest;
use crate::domain::use_cases::scope::ScopeUseCaseResponse;

// ================ Create application ================

#[derive(Debug, Clone)]
pub struct CreateScopeDtoRequest(pub CreateScopeUseCaseRequest);

#[derive(Debug, Clone)]
pub struct CreateScopeDtoResponse(pub ScopeUseCaseResponse);
