use std::env;

pub struct MailgunConfig {
    url: String,
    api_user: String,
    api_key: String,
    default_to: String,
    from: String,
}

impl MailgunConfig {
    pub fn from_env() -> Self {
        Self {
            url: env::var("MAILGUN_URL").expect("MAILGUN_URL env not set"),
            api_user: env::var("MAILGUN_API_USER").expect("MAILGUN_API_USER env not set"),
            api_key: env::var("MAILGUN_API_KEY").expect("MAILGUN_API_KEY env not set"),
            default_to: env::var("MAILGUN_DEFAULT_TO").expect("MAILGUN_DEFAULT_TO env not set"),
            from: env::var("MAILGUN_FROM").expect("MAILGUN_FROM env not set"),
        }
    }
}