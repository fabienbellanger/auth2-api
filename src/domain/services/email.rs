//! Email service

use crate::domain::entities::email::EmailMessage;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
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
}

pub trait EmailTransport {
    /// Send an email
    fn send(&self, message: EmailMessage) -> Result<(), EmailServiceError>;
}

/// List all email services
pub trait EmailService {
    // fn forgotten_password(&self, request: ForgottenPasswordRequest, token: &str) -> Result<(), EmailServiceError>;
}
