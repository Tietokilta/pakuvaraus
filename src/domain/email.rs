use std::str::FromStr;

use super::ParseError;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
#[serde(try_from = "String")]
pub struct Email(String);

impl Email {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Email {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let email = s.trim().to_ascii_lowercase();
        garde::rules::email::parse_email(&email).map_err(|_| ParseError::Email)?;
        Ok(Self(email))
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.as_str(), f)
    }
}

impl TryFrom<String> for Email {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
