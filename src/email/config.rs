use std::env;

use crate::util::env::parse_bool;

pub struct EmailConfig {
    pub enabled: bool,
}

impl EmailConfig {
    pub fn from_env() -> Self {
        Self {
            enabled: parse_bool(&env::var("EMAIL_ENABLED").expect("EMAIL_ENABLED env not set")),
        }
    }
}