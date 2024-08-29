//! List of user use cases
//!
pub mod create_user;
pub mod get_access_token;

use crate::domain::repositories::user::UserRepository;
use crate::domain::use_cases::user::get_access_token::GetAccessTokenUseCase;
use create_user::CreateUserUseCase;

#[derive(Debug, Clone)]
pub struct UserUseCases<U: UserRepository> {
    pub create_user: CreateUserUseCase<U>,
    pub get_access_token: GetAccessTokenUseCase<U>,
}

impl<U: UserRepository> UserUseCases<U> {
    /// Create a new user use cases
    pub fn new(user_repository: U) -> Self {
        Self {
            create_user: CreateUserUseCase::new(user_repository.clone()),
            get_access_token: GetAccessTokenUseCase::new(user_repository),
        }
    }
}
