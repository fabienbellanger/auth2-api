//! DTO for scope repository

use crate::domain::use_cases::scope::create_scope::CreateScopeUseCaseRequest;
use crate::domain::use_cases::scope::get_scopes::GetScopesUseCaseRequest;
use crate::domain::use_cases::scope::ScopeUseCaseResponse;

// ================ Create application ================

#[derive(Debug, Clone)]
pub struct CreateScopeDtoRequest(pub CreateScopeUseCaseRequest);

#[derive(Debug, Clone)]
pub struct CreateScopeDtoResponse(pub ScopeUseCaseResponse);

// ================ Get all scopes ================

#[derive(Debug, Clone)]
pub struct GetScopesDtoRequest(pub GetScopesUseCaseRequest);

#[derive(Debug, Clone)]
pub struct GetScopesDtoResponse(pub Vec<ScopeUseCaseResponse>);

// ================ Count scopes ================

#[derive(Debug, Clone)]
pub struct CountScopesDtoRequest {
    pub deleted: bool,
}

#[derive(Debug, Clone)]
pub struct CountScopesDtoResponse(pub i64);
