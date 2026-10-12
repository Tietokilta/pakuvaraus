use async_trait::async_trait;
use reqwest::{
    Client,
    multipart::{Form, Part},
};

use super::config::MailgunConfig;
use crate::email::{EmailError, attachment::Attachment, service::EmailSender};

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
    async fn send(
        &self,
        to: String,
        subject: String,
        html: String,
        attachments: Box<[Attachment<'_>]>,
    ) -> Result<(), EmailError> {
        let mut form = Form::new()
            .text("from", self.from.clone())
            .text("to", to)
            .text("subject", subject)
            .text("html", html);

        for att in attachments {
            form = form.part(
                "attachment",
                Part::bytes(att.data)
                    .file_name(att.filename)
                    .mime_str(att.content_type)?,
            );
        }

        let res = self
            .client
            .post(&self.url)
            .basic_auth(&self.api_user, Some(&self.api_key))
            .multipart(form)
            .send()
            .await?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await?;
            return Err(EmailError::Rejected { status, body });
        }
        Ok(())
    }
}
