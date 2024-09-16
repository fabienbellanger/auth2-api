//! Application use cases

pub mod create_application;

use crate::domain::entities::application::ApplicationId;
use crate::domain::value_objects::datetime::UtcDateTime;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ApplicationUseCases {}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ApplicationUseCaseError {}

/// Application use case generic response
#[derive(Debug, Clone)]
pub struct ApplicationUseCaseResponse {
    pub id: ApplicationId,
    pub name: String,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
}
