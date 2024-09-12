//! Send email on forgotten password implementation

use crate::domain::entities::email::EmailMessage;
use crate::domain::services::email::EmailServiceError;
use validator::ValidateUrl;

#[derive(Debug, Clone)]
pub struct ForgottenPassword {
    pub app_name: String,
    pub base_url: String,
    pub token: String,
    pub email_from: String,
    pub email_to: String,
}

impl ForgottenPassword {
    /// Get email subject
    fn subject(&self) -> String {
        format!("{} - Forgotten password", self.app_name)
    }

    /// Get password reset link
    fn link(&self) -> Result<String, EmailServiceError> {
        let link = format!("{}/{}", self.base_url, self.token);

        match ValidateUrl::validate_url(&link) {
            true => Ok(link),
            false => Err(EmailServiceError::InvalidParameter(format!("invalid URL: {link}"))),
        }
    }

    /// Construct TEXT body
    // TODO: Use Tera instead?
    fn construct_text_body(&self, link: &str) -> Result<String, EmailServiceError> {
        Ok(format!(
            r#"Forgotten password
==================

You told us you forgot your password. If you really did, click here to choose a new one:

{link}

If you didn't mean to reset your password, then you can just ignore this email; your password will not change."#
        ))
    }

    /// Construct HTML body
    // TODO: Use Tera instead?
    fn construct_html_body(&self, link: &str) -> Result<String, EmailServiceError> {
        Ok(format!(
            r#"
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link
    href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap"
    rel="stylesheet">
</head>
<body style="margin: 16px; color: #212121; font-size: 13px; font-weight: 400; font-family: 'Roboto', sans-serif">
  <h1 style="font-size: 24px; font-weight: 600">Forgotten password</h1>
  <section>
    <p>You told us you forgot your password. If you really did, click here to choose a new one:</p>
    <a href="{link}"
      style="display: inline-block; background-color: #1976D2; color: white; padding: 16px 24px; text-decoration: none; margin: 16px; text-align: center; font-size: 16px">
      Choose a new password
    </a>
    <p>
      If you didn't mean to reset your password, then you can just ignore this email; your password will not change.
    </p>
  </section>
</body>"#
        ))
    }
}

impl TryInto<EmailMessage> for ForgottenPassword {
    type Error = EmailServiceError;

    fn try_into(self) -> Result<EmailMessage, Self::Error> {
        let link = self.link()?;

        Ok(EmailMessage {
            from_address: self.email_from.clone(),
            subject: self.subject(),
            text_body: Some(self.construct_text_body(&link)?),
            html_body: Some(self.construct_html_body(&link)?),
            to_addresses: vec![self.email_to],
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::entities::email::EmailMessage;

    #[test]
    fn test_forgotten_password_request_subject() {
        let request = ForgottenPassword {
            app_name: "My App".to_owned(),
            base_url: "https://test.com".to_owned(),
            token: "myToken5846".to_owned(),
            email_from: "from@test.com".to_owned(),
            email_to: "to@test.com".to_owned(),
        };

        assert_eq!(request.subject(), "My App - Forgotten password".to_owned());
    }

    #[test]
    fn test_forgotten_password_request_link() {
        let request = ForgottenPassword {
            app_name: "My App".to_owned(),
            base_url: "https://test.com".to_owned(),
            token: "myToken5846".to_owned(),
            email_from: "from@test.com".to_owned(),
            email_to: "to@test.com".to_owned(),
        };
        assert_eq!(request.link().unwrap(), "https://test.com/myToken5846".to_owned());

        let request = ForgottenPassword {
            app_name: "My App".to_owned(),
            base_url: "-test.com".to_owned(),
            token: "myToken5846".to_owned(),
            email_from: "from@test.com".to_owned(),
            email_to: "to@test.com".to_owned(),
        };
        assert!(request.link().is_err());
    }

    #[test]
    fn test_forgotten_password_request_try_into_message() {
        let request = ForgottenPassword {
            app_name: "My App".to_owned(),
            base_url: "https://test.com".to_owned(),
            token: "myToken5846".to_owned(),
            email_from: "from@test.com".to_owned(),
            email_to: "to@test.com".to_owned(),
        };
        let msg: EmailMessage = request.try_into().unwrap();

        assert_eq!(msg.subject, "My App - Forgotten password".to_owned());
        assert_eq!(msg.from_address, "from@test.com".to_owned());
        assert_eq!(msg.to_addresses, vec!["to@test.com".to_owned()]);
        assert!(msg.text_body.is_some());
        assert!(msg.html_body.is_some());
    }
}
