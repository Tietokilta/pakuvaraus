use dotenv::dotenv;

use crate::mailgun::config::MailgunConfig;
use crate::db::config::DbConfig;

pub struct AppConfig {
    pub mailgun: MailgunConfig,
    pub db: DbConfig,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            mailgun: MailgunConfig::from_env(),
            db: DbConfig::from_env(),
        }
    }
}