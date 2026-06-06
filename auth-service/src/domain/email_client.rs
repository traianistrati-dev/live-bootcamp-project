use super::email::Email;

use color_eyre::eyre::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailError {
    // #[error("Invalid credentials")]
    // InvalidCredentials,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

// This trait represents the interface all concrete email clients should implement
#[async_trait::async_trait]
pub trait EmailClient {
    async fn send_email(
        &self,
        recipient: &Email,
        subject: &str,
        content: &str,
    ) -> Result<(), EmailError>;
}
