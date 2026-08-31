use async_trait::async_trait;

use super::config::EmailConfig;
use crate::mailgun::service::MailgunService;

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send(&self, to: &str, subject: &str, html: &str) -> Result<(), String>;
}

impl EmailService {
    pub async fn from_config(config: &EmailConfig) -> Self {
        if(config.enabled) {
            return MailgunService::create();
        }
    }

    pub async fn create() -> Self {
        Self::from_config(&&EmailConfig::from_env()).await
    }
}