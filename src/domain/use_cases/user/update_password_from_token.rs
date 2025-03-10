//! Update user password from forgotten password request

use crate::domain::entities::password_reset::PasswordResetTokenValue;
use crate::domain::repositories::password_reset::PasswordResetRepository;
use crate::domain::repositories::password_reset::dto::{DeletePasswordResetDtoRequest, GetUserIdFromTokenDtoRequest};
use crate::domain::repositories::user::UserRepository;
use crate::domain::repositories::user::dto::UpdatePasswordDtoRequest;
use crate::domain::use_cases::user::UserUseCaseError;
use crate::domain::value_objects::password::Password;

#[derive(Debug, Clone)]
pub struct UpdatePasswordFromTokenUseCaseRequest {
    pub token: PasswordResetTokenValue,
    pub password: Password,
}

#[derive(Debug, Clone)]
pub struct UpdatePasswordFromTokenUseCaseResponse();

#[derive(Debug, Clone)]
pub struct UpdatePasswordFromTokenUseCase<U: UserRepository, P: PasswordResetRepository> {
    user_repository: U,
    password_reset_repository: P,
}

impl<U: UserRepository, P: PasswordResetRepository> UpdatePasswordFromTokenUseCase<U, P> {
    /// Create a new use case
    pub fn new(user_repository: U, password_reset_repository: P) -> Self {
        Self {
            user_repository,
            password_reset_repository,
        }
    }

    /// Update user password from forgotten password request token
    // TODO: Check if the password is not the same as the previous one
    #[instrument(skip(self), name = "update_password_from_token_use_case")]
    pub async fn call(
        &self,
        request: UpdatePasswordFromTokenUseCaseRequest,
    ) -> Result<UpdatePasswordFromTokenUseCaseResponse, UserUseCaseError> {
        // Get user ID from token
        let user_id = self
            .password_reset_repository
            .get_user_from_token(GetUserIdFromTokenDtoRequest { token: request.token })
            .await?
            .user_id
            .ok_or_else(UserUseCaseError::ForgottenPasswordNotFound)?;

        // Update user password
        self.user_repository
            .update_password(UpdatePasswordDtoRequest {
                user_id: user_id.clone(),
                password: request.password,
            })
            .await?;

        // Delete password reset
        self.password_reset_repository
            .delete(DeletePasswordResetDtoRequest { user_id })
            .await?;

        Ok(UpdatePasswordFromTokenUseCaseResponse())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::mock::password_reset::{INVALID_TOKEN, PasswordResetRepositoryMock, VALID_TOKEN};
    use crate::domain::tests::mock::user::UserRepositoryMock;

    #[tokio::test]
    async fn test_update_password_from_token_use_case() {
        let user_repository = UserRepositoryMock {};
        let password_reset_repository = PasswordResetRepositoryMock {};
        let use_case = UpdatePasswordFromTokenUseCase::new(user_repository, password_reset_repository);

        let response = use_case
            .call(UpdatePasswordFromTokenUseCaseRequest {
                token: VALID_TOKEN.into(),
                password: Password::new("newPassword123", false).unwrap(),
            })
            .await;
        assert!(response.is_ok());

        let response = use_case
            .call(UpdatePasswordFromTokenUseCaseRequest {
                token: INVALID_TOKEN.into(),
                password: Password::new("newPassword123", false).unwrap(),
            })
            .await;
        assert!(response.is_err());
    }
}
