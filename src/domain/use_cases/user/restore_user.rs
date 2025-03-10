//! Restore a deleted user

use crate::domain::entities::user::UserId;
use crate::domain::repositories::user::UserRepository;
use crate::domain::repositories::user::dto::RestoreUserDtoRequest;
use crate::domain::use_cases::user::UserUseCaseError;

#[derive(Debug, Clone)]
pub struct RestoreUserUseCaseRequest {
    pub user_id: UserId,
}

#[derive(Debug, Clone)]
pub struct RestoreUserUseCaseResponse();

#[derive(Debug, Clone)]
pub struct RestoreUserUseCase<U: UserRepository> {
    user_repository: U,
}

impl<U: UserRepository> RestoreUserUseCase<U> {
    /// Create a new use case
    pub fn new(user_repository: U) -> Self {
        Self { user_repository }
    }

    /// Restore a user
    #[instrument(skip(self), name = "restore_user_use_case")]
    pub async fn call(
        &self,
        request: RestoreUserUseCaseRequest,
    ) -> Result<RestoreUserUseCaseResponse, UserUseCaseError> {
        // TODO: Validation?

        let result = self
            .user_repository
            .restore_user(RestoreUserDtoRequest(request))
            .await?;

        Ok(result.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::mock::user::{INVALID_ID, UserRepositoryMock, VALID_ID};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_delete_user_use_case() {
        let user_repository = UserRepositoryMock {};
        let use_case = RestoreUserUseCase::new(user_repository);

        let request = RestoreUserUseCaseRequest {
            user_id: UserId::from_str(VALID_ID).unwrap(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_delete_user_use_case_not_found() {
        let user_repository = UserRepositoryMock {};
        let use_case = RestoreUserUseCase::new(user_repository);

        let request = RestoreUserUseCaseRequest {
            user_id: UserId::from_str(INVALID_ID).unwrap(),
        };

        let response = use_case.call(request).await;
        assert!(response.is_err());
        if let Err(err) = response {
            assert_eq!(err, UserUseCaseError::DatabaseError("User not found".to_string()));
        }
    }
}
