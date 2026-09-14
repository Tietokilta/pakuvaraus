use async_trait::async_trait;
use reqwest::{Client, multipart::Form};

use super::config::MailgunConfig;
use crate::email::service::EmailSender;

#[derive(Clone)]
pub struct MailgunService {
    client: Client,
    url: String,
    api_user: String,
    api_key: String,
    from: String,
}

impl MailgunService {
    pub fn from_config(config: &MailgunConfig) -> Self {
        Self {
            client: Client::new(),
            url: config.url.clone(),
            api_user: config.api_user.clone(),
            api_key: config.api_key.clone(),
            from: config.from.clone(),
        }
    }

    pub fn create() -> Self {
        Self::from_config(&MailgunConfig::from_env())
    }
}

#[async_trait]
impl EmailSender for MailgunService {
    async fn send(&self, to: &str, subject: &str, html: &str) -> Result<(), String> {
        let form = Form::new()
            .text("from", self.from.clone())
            .text("to", to.to_string())
            .text("subject", subject.to_string())
            .text("html", html.to_string());

        let res = self
            .client
            .post(self.url.clone())
            .basic_auth(self.api_user.clone(), Some(self.api_key.clone()))
            .multipart(form)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            return Err(format!("Error: {}", res.status()));
        }

        Ok(())
    }
}
