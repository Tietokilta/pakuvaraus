use async_trait::async_trait;
use tracing::info;

use crate::email::{EmailError, attachment::Attachment};

use super::service::EmailSender;

#[derive(Clone)]
pub struct StubService {}

impl StubService {
    pub const fn create() -> Self {
        Self {}
    }
}

#[async_trait]
impl EmailSender for StubService {
    async fn send(
        &self,
        to: String,
        subject: String,
        html: String,
        attachments: Box<[Attachment]>,
    ) -> Result<(), EmailError> {
        info!(
            "Send email to: {to}, subject: {subject}, {} attachments, html: {html}",
            attachments.len()
        );
        Ok(())
    }
}
