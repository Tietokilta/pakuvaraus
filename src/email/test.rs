use async_trait::async_trait;
use tracing::info;

use crate::email::EmailError;

use super::service::EmailSender;

#[derive(Clone)]
pub struct StubService {

}

impl StubService {
    pub const fn create() -> Self {
        Self { }
    }
}

#[async_trait]
impl EmailSender for StubService {
    async fn send(&self, to: &str, subject: &str, html: &str) -> Result<(), EmailError> {
        info!("Send email to: {}, subject: {}, html: {}", to, subject, html);
        Ok(())
    }
}
