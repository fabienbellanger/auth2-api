//! Send email on forgotten password request

use crate::domain::entities::password_reset::PasswordResetTokenValue;
use crate::domain::value_objects::email::Email;

#[derive(Debug, Clone)]
pub struct ForgottenPasswordEmailRequest {
    pub email: Email,
    pub token: PasswordResetTokenValue,
}

#[derive(Debug, Clone)]
pub struct ForgottenPasswordEmailResponse();
