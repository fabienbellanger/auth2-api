//! Services are the core of the domain layer.
//! They are the business logic of the application.

pub mod email;
pub mod security;

use crate::domain::repositories::user::UserRepository;
use crate::domain::use_cases::user::UserUseCases;

/// List of services
#[derive(Debug, Clone)]
pub struct Service<U>
where
    U: UserRepository,
{
    pub user_use_cases: UserUseCases<U>,
}

impl<U> Service<U>
where
    U: UserRepository,
{
    /// Create new use cases
    pub fn new(repositories: U) -> Self {
        Self {
            user_use_cases: UserUseCases::new(repositories),
        }
    }
}
