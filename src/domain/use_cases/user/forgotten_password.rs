//! Forgotten password use case

use crate::domain::entities::password_reset::PasswordReset;
use crate::domain::repositories::password_reset::PasswordResetRepository;
use crate::domain::repositories::password_reset::dto::CreateUpdatePasswordResetDtoRequest;
use crate::domain::repositories::user::UserRepository;
use crate::domain::repositories::user::dto::GetUserByEmailDtoRequest;
use crate::domain::services::email::EmailService;
use crate::domain::services::email::forgotten_password::ForgottenPasswordEmailRequest;
use crate::domain::use_cases::user::UserUseCaseError;
use crate::domain::value_objects::email::Email;

#[derive(Debug, Clone)]
pub struct ForgottenPasswordUseCaseRequest {
    pub user_email: Email,
    pub expiration_duration: i64,
}

#[derive(Debug, Clone)]
pub struct ForgottenPasswordUseCaseResponse(pub PasswordReset);

#[derive(Debug, Clone)]
pub struct ForgottenPasswordUseCase<U: UserRepository, P: PasswordResetRepository, E: EmailService> {
    user_repository: U,
    password_reset_repository: P,
    email_service: E,
}

impl<U: UserRepository, P: PasswordResetRepository, E: EmailService> ForgottenPasswordUseCase<U, P, E> {
    /// Create a new use case
    pub fn new(user_repository: U, password_reset_repository: P, email_service: E) -> Self {
        Self {
            user_repository,
            password_reset_repository,
            email_service,
        }
    }

    /// Generate a token to generate a new password and send it to the user by email
    #[instrument(skip(self), name = "forgotten_password_use_case")]
    pub async fn call(
        &self,
        request: ForgottenPasswordUseCaseRequest,
    ) -> Result<ForgottenPasswordUseCaseResponse, UserUseCaseError> {
        // Get user by email
        let user = self
            .user_repository
            .get_user_by_email(GetUserByEmailDtoRequest(request.clone()))
            .await?;

        // Create a new password reset entry
        let password_reset = PasswordReset::new(user.0.id.clone(), request.expiration_duration);
        self.password_reset_repository
            .create_or_update(CreateUpdatePasswordResetDtoRequest(password_reset.clone()))
            .await?;

        // Send email with the token to the user
        self.email_service
            .forgotten_password(ForgottenPasswordEmailRequest {
                email: user.0.email.clone(),
                token: password_reset.token.clone(),
            })
            .map_err(|err| UserUseCaseError::SendEmailError(err.to_string()))?;

        Ok(ForgottenPasswordUseCaseResponse(password_reset))
    }
}
