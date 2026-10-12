use std::env;

use crate::{domain::Email, util::env::parse_bool};

#[derive(Clone, Debug)]
pub struct EmailConfig {
    pub enabled: bool,
    pub approver_email: Email,
}

impl EmailConfig {
    pub fn from_env() -> Self {
        let approver_email = env::var("APPROVER_EMAIL")
            .expect("APPROVER_EMAIL env not set")
            .try_into()
            .expect("APPROVER_EMAIL env is not a valid email");
        Self {
            enabled: parse_bool(&env::var("EMAIL_ENABLED").expect("EMAIL_ENABLED env not set")),
            approver_email,
        }
    }
}
