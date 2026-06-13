use secrecy::ExposeSecret;

use crate::domain::{email::Email, email_client::EmailClient};

//use color_eyre::eyre::{eyre, Context, Report, Result};

#[derive(Default)]
pub struct MockEmailClient;

#[async_trait::async_trait]
impl EmailClient for MockEmailClient {
    async fn send_email(
        &self,
        recipient: &Email,
        subject: &str,
        content: &str,
    ) -> color_eyre::eyre::Result<()> {
        // Our mock email client will simply log the recipient, subject, and content to standard output
        // println!(
        //     "Sending email to {} with subject: {} and content: {}",
        //     recipient.as_ref(),
        //     subject,
        //     content
        // );

        tracing::debug!(
            "Sending email to {} with subject: {} and content: {}",
            recipient.as_ref().expose_secret(),
            subject,
            content
        );

        Ok(())
    }
}
