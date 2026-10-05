use super::ParseError;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, sqlx::Type)]
#[serde(try_from = "String")]
#[sqlx(transparent)]
pub struct NonEmptyText<const N: usize>(String);

impl<const N: usize> NonEmptyText<N> {
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        let s = s.trim();
        if s.is_empty() || s.chars().count() > N {
            return Err(ParseError::TextLength(N));
        }
        Ok(Self(s.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<const N: usize> Into<String> for NonEmptyText<N> {
    fn into(self) -> String {
        self.0
    }
}

impl<const N: usize> std::fmt::Display for NonEmptyText<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<const N: usize> TryFrom<String> for NonEmptyText<N> {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}
