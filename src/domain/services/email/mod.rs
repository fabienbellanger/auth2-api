//! Email service

pub mod forgotten_password;

use crate::domain::entities::email::EmailMessage;
use crate::domain::services::email::forgotten_password::{
    ForgottenPasswordEmailRequest, ForgottenPasswordEmailResponse,
};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum EmailServiceError {
    #[error("Send email error: {0}")]
    SendError(String),

    #[error("Email configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid From email: {0}")]
    InvalidEmailFrom(String),

    #[error("Invalid To email: {0}")]
    InvalidEmailTo(String),

    #[error("Multipart error: {0}")]
    MultiPartError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

pub trait EmailTransport {
    /// Send an email
    fn send(&self, message: EmailMessage) -> Result<(), EmailServiceError>;
}

/// List all email services
pub trait EmailService {
    /// Send email for a forgotten password request
    fn forgotten_password(
        &self,
        request: ForgottenPasswordEmailRequest,
    ) -> Result<ForgottenPasswordEmailResponse, EmailServiceError>;
}
