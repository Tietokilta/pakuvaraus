use std::env;

pub struct MailgunConfig {
    pub url: String,
    pub api_user: String,
    pub api_key: String,
    pub from: String,
}

impl MailgunConfig {
    pub fn from_env() -> Self {
        Self {
            url: env::var("MAILGUN_URL").expect("MAILGUN_URL env not set"),
            api_user: env::var("MAILGUN_API_USER").expect("MAILGUN_API_USER env not set"),
            api_key: env::var("MAILGUN_API_KEY").expect("MAILGUN_API_KEY env not set"),
            from: env::var("MAILGUN_FROM").expect("MAILGUN_FROM env not set"),
        }
    }
}