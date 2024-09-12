//! Email entity

use crate::config::Config;

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

    /// Forgotten password link base URL
    pub forgotten_password_base_url: String,

    /// Forgotten password email from
    pub forgotten_password_email_from: String,
}

impl From<Config> for EmailConfig {
    fn from(config: Config) -> Self {
        EmailConfig {
            host: config.smtp_host,
            port: config.smtp_port,
            timeout: config.smtp_timeout,
            username: match config.smtp_username.is_empty() {
                false => Some(config.smtp_username),
                true => None,
            },
            password: match config.smtp_password.is_empty() {
                false => Some(config.smtp_password),
                true => None,
            },
            forgotten_password_base_url: config.forgotten_password_base_url,
            forgotten_password_email_from: config.forgotten_password_email_from,
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
