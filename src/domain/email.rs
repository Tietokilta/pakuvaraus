use super::ParseError;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
#[serde(try_from = "String")]
pub struct Email(String);

impl Email {
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        let email = s.trim().to_ascii_lowercase();
        garde::rules::email::parse_email(&email).map_err(|_| ParseError::Email)?;
        Ok(Self(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Email {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}
