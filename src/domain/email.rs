#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

pub enum EmailError {
    Invalid,
}

impl Email {
    pub fn parse(s: &str) -> Result<Self, EmailError> {
        if validator::ValidateEmail::validate_email(&s) {
            Ok(Self(s.to_string()))
        } else {
            Err(EmailError::Invalid)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Email {
    fn from(s: String) -> Self {
        Self(s)
    }
}
