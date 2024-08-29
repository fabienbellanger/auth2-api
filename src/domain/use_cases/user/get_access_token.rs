//! Get Access Token Use Case

use crate::domain::entities::access_token::AccessTokenValue;
use crate::domain::repositories::user::UserRepository;
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;
use thiserror::Error;

/// Get access token errors
#[derive(Debug, Clone, PartialEq, Error)]
pub enum GetAccessTokenError {}

// TODO: Add application_id and client_id later
#[derive(Debug, Clone)]
pub struct GetAccessTokenUseCaseRequest {
    /// User email
    pub email: Email,

    /// User password
    pub password: Password,
}

// TODO: Add refresh_token information later
#[derive(Debug, Clone)]
pub struct GetAccessTokenUseCaseResponse {
    /// Access token value
    pub access_token: AccessTokenValue,

    /// Access token expiration time
    pub access_token_expired_at: UtcDateTime,
}

#[derive(Debug, Clone)]
pub struct GetAccessTokenUseCase<U: UserRepository> {
    #[allow(dead_code)]
    user_repository: U,
}

impl<U: UserRepository> GetAccessTokenUseCase<U> {
    /// Create a new use case
    pub fn new(user_repository: U) -> Self {
        Self { user_repository }
    }

    /// Generate a new access token
    #[instrument(skip(self), name = "get_access_token_use_case")]
    pub async fn call(
        &self,
        _request: GetAccessTokenUseCaseRequest,
    ) -> Result<GetAccessTokenUseCaseResponse, GetAccessTokenError> {
        // TODO: Validation?

        unimplemented!()
    }
}
