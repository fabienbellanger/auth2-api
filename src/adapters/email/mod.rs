//! Email adapter using `lettre` crate

use crate::domain::entities::email::{EmailConfig, EmailMessage};
use crate::domain::services::email::{EmailServiceError, EmailTransport};
use lettre::address::AddressError;
use lettre::message::{header, MultiPart, SinglePart};
use lettre::{SmtpTransport, Transport};
use std::time::Duration;

#[derive(Debug, Default, Clone)]
pub struct EmailAdapter {
    config: EmailConfig,
}

impl EmailAdapter {
    /// New email with SMTP config
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }

    /// Create SMTP transport
    pub fn smtp_transport(&self) -> SmtpTransport {
        let timeout = match self.config.timeout {
            0 => None,
            t => Some(Duration::from_secs(t)),
        };

        // TODO: Manage SMTP username and password
        SmtpTransport::builder_dangerous(&self.config.host)
            .port(self.config.port)
            .timeout(timeout)
            .build()
    }
}

impl EmailTransport for EmailAdapter {
    /// Send an email
    fn send(&self, message: EmailMessage) -> Result<(), EmailServiceError> {
        let mailer = self.smtp_transport();

        let mut email_builder = lettre::Message::builder().subject(message.subject).from(
            message
                .from_address
                .parse()
                .map_err(|err: AddressError| EmailServiceError::InvalidEmailFrom(err.to_string()))?,
        );

        // Add destination emails
        for to in message.to_addresses {
            email_builder = email_builder.to(to
                .parse()
                .map_err(|err: AddressError| EmailServiceError::InvalidEmailTo(err.to_string()))?)
        }

        let mut multipart = MultiPart::alternative().build();
        if let Some(text) = message.text_body {
            multipart = multipart.singlepart(SinglePart::builder().header(header::ContentType::TEXT_PLAIN).body(text));
        }
        if let Some(html) = message.html_body {
            multipart = multipart.singlepart(SinglePart::builder().header(header::ContentType::TEXT_HTML).body(html));
        }
        let email = email_builder
            .multipart(multipart)
            .map_err(|err| EmailServiceError::MultiPartError(err.to_string()))?;

        mailer
            .send(&email)
            .map_err(|err| EmailServiceError::SendError(err.to_string()))?;

        Ok(())
    }
}
