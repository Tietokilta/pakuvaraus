use std::sync::Mutex;

use async_trait::async_trait;
use pakuvaraus::email::{EmailError, attachment::Attachment, service::EmailSender};

pub const APPROVER_EMAIL: &str = "pakuvastaava@tietokilta.fi";

#[derive(PartialEq, Eq)]
pub struct SentMail {
    pub to: String,
    pub attachments: Box<[Attachment]>,
}

pub struct TestSender {
    pub sent_mails: Mutex<Vec<SentMail>>,
}

impl TestSender {
    pub fn new() -> Self {
        Self {
            sent_mails: Mutex::default(),
        }
    }
}

#[async_trait]
impl EmailSender for TestSender {
    async fn send(
        &self,
        to: String,
        _subject: String,
        _html: String,
        attachments: Box<[Attachment]>,
    ) -> Result<(), EmailError> {
        self.sent_mails
            .lock()
            .unwrap()
            .push(SentMail { to, attachments });
        Ok(())
    }
}
