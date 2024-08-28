//! List of user use cases
pub mod create_user;

use crate::domain::repositories::user::UserRepository;
use create_user::CreateUserUseCase;

#[derive(Debug, Clone)]
pub struct UserUseCases<U: UserRepository> {
    pub create_user: CreateUserUseCase<U>,
}

impl<U: UserRepository> UserUseCases<U> {
    /// Create a new user use cases
    pub fn new(user_repository: U) -> Self {
        Self {
            create_user: CreateUserUseCase::new(user_repository),
        }
    }
}
