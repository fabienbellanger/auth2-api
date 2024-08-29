//! Email entity

pub type EmailAddress = String;

#[derive(Debug, Default, Clone)]
pub struct EmailConfig {
    /// SMTP host
    pub host: String,

    /// SMTP port
    pub port: u16,

    /// SMTP timeout
    pub timeout: u64,

    /// SMTP username
    pub username: Option<String>,

    /// SMTP password
    pub password: Option<String>,
}

impl EmailConfig {
    /// New email configuration without authentication
    pub fn new(host: String, port: u16, timeout: u64) -> Self {
        Self {
            host,
            port,
            timeout,
            username: None,
            password: None,
        }
    }
    /// New email configuration with authentication
    pub fn new_with_auth(host: String, port: u16, timeout: u64, username: String, password: String) -> Self {
        Self {
            host,
            port,
            timeout,
            username: Some(username),
            password: Some(password),
        }
    }
}

/// Message to send
#[derive(Debug, Default, Clone)]
pub struct EmailMessage {
    pub from_address: EmailAddress,
    pub to_addresses: Vec<EmailAddress>,
    pub subject: String,
    pub text_body: Option<String>,
    pub html_body: Option<String>,
}
