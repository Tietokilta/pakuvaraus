use async_trait::async_trait;
use std::sync::Arc;

use super::{config::EmailConfig, test::StubService};
use crate::mailgun::service::MailgunService;

#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send(&self, to: &str, subject: &str, html: &str) -> Result<(), String>;
}

#[derive(Clone)]
pub struct EmailService {
    pub sender: Arc<dyn EmailSender>
}

impl EmailService {
    pub async fn from_config(config: &EmailConfig) -> Self {
        let sender: Arc<dyn EmailSender> = if config.enabled {
            Arc::new(MailgunService::create().await)
        } else {
            Arc::new(StubService::create())
        };

        Self {
            sender: sender
        }
    }

    pub async fn create() -> Self {
        Self::from_config(&&EmailConfig::from_env()).await
    }
}