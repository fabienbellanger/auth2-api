//! Get all users with pagination use case

use crate::domain::repositories::user::dto::GetUsersDtoRequest;
use crate::domain::repositories::user::UserRepository;
use crate::domain::use_cases::user::{UserUseCaseError, UserUseCaseResponse};
use crate::domain::utils::query_sort::Filter;

#[derive(Debug, Clone)]
pub struct GetUsersUseCaseRequest {
    pub filter: Option<Filter>,
}

#[derive(Debug, Clone)]
pub struct GetUsersUseCaseResponse {
    pub users: Vec<UserUseCaseResponse>,
    pub total: i64,
}

#[derive(Debug, Clone)]
pub struct GetUsersUseCase<U: UserRepository> {
    user_repository: U,
}

impl<U: UserRepository> GetUsersUseCase<U> {
    /// Create a new use case
    pub fn new(user_repository: U) -> Self {
        Self { user_repository }
    }

    /// Get all users
    #[instrument(skip(self), name = "get_users_use_case")]
    pub async fn call(&self, request: GetUsersUseCaseRequest) -> Result<GetUsersUseCaseResponse, UserUseCaseError> {
        // TODO: Validation?

        let total = self.user_repository.count_users(()).await?.0;
        let users = self.user_repository.get_users(GetUsersDtoRequest(request)).await?.0;

        Ok(GetUsersUseCaseResponse { users, total })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::mock::user::UserRepositoryMock;
    use crate::domain::utils::query_sort::Sorts;
    use crate::domain::value_objects::pagination::Pagination;

    #[tokio::test]
    async fn test_get_users_use_case() {
        let user_repository = UserRepositoryMock {};
        let use_case = GetUsersUseCase::new(user_repository);

        let request = GetUsersUseCaseRequest {
            filter: Some(Filter {
                pagination: Some(Pagination::new(1, 10)),
                sorts: Some(Sorts::default()),
            }),
        };

        let response = use_case.call(request).await;
        assert!(response.is_ok());
    }
}
