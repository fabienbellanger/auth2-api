//! Get user from its ID

use crate::domain::entities::user::UserId;
use crate::domain::repositories::user::UserRepository;
use crate::domain::repositories::user::dto::GetUserByIdDtoRequest;
use crate::domain::use_cases::user::{UserUseCaseError, UserUseCaseResponse};

#[derive(Debug, Clone)]
pub struct GetUserUseCaseRequest {
    pub user_id: UserId,
}

#[derive(Debug, Clone)]
pub struct GetUserUseCaseResponse(pub UserUseCaseResponse);

#[derive(Debug, Clone)]
pub struct GetUserUseCase<U: UserRepository> {
    user_repository: U,
}

impl<U: UserRepository> GetUserUseCase<U> {
    /// Create a new use case
    pub fn new(user_repository: U) -> Self {
        Self { user_repository }
    }

    /// Get a user
    #[instrument(skip(self), name = "get_user_use_case")]
    pub async fn call(&self, request: GetUserUseCaseRequest) -> Result<GetUserUseCaseResponse, UserUseCaseError> {
        // TODO: Validation?

        let user = self
            .user_repository
            .get_user_by_id(GetUserByIdDtoRequest(request))
            .await?
            .0;

        Ok(GetUserUseCaseResponse(user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::mock::user::{INVALID_ID, UserRepositoryMock, VALID_ID};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_get_user_use_case() {
        let user_repository = UserRepositoryMock {};
        let use_case = GetUserUseCase::new(user_repository);

        let request = GetUserUseCaseRequest {
            user_id: UserId::from_str(VALID_ID).unwrap(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_use_case_not_found() {
        let user_repository = UserRepositoryMock {};
        let use_case = GetUserUseCase::new(user_repository);

        let request = GetUserUseCaseRequest {
            user_id: UserId::from_str(INVALID_ID).unwrap(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_err());
        if let Err(err) = response {
            assert_eq!(err, UserUseCaseError::DatabaseError("User not found".to_string()));
        }
    }
}
