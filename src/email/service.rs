use async_trait::async_trait;

use super::{config::EmailConfig, test::StubService};
use crate::mailgun::service::MailgunService;

#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send(&self, to: &str, subject: &str, html: &str) -> Result<(), String>;
}

pub struct EmailService {
    pub sender: Box<dyn EmailSender>
}

impl EmailService {
    pub async fn from_config(config: &EmailConfig) -> Self {
        let sender: Box<dyn EmailSender> = if config.enabled {
            Box::new(MailgunService::create().await)
        } else {
            Box::new(StubService::create())
        };

        Self {
            sender: sender
        }
    }

    pub async fn create() -> Self {
        Self::from_config(&&EmailConfig::from_env()).await
    }
}