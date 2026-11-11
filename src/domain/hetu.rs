use std::fmt;

use super::ParseError;
use hetu::Ssn;
use serde::Deserialize;
use utoipa::openapi::{ObjectBuilder, RefOr, Schema, Type};

#[derive(Clone, Deserialize, PartialEq, Eq)]
#[serde(try_from = "String")]
pub struct Hetu(String);

impl Hetu {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn parse(s: impl Into<String>) -> Result<Self, ParseError> {
        let mut s = s.into();
        s.make_ascii_uppercase();
        Ssn::try_from(s.as_str())
            .map_err(|_| ParseError::Hetu)
            .map(|_| Self(s))
    }
}

impl utoipa::PartialSchema for Hetu {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new()
            .schema_type(Type::String)
            .description(Some("Finnish personal identity code (henkilötunnus)"))
            .into()
    }
}
impl utoipa::ToSchema for Hetu {}

impl TryFrom<String> for Hetu {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl fmt::Debug for Hetu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Hetu(redacted)")
    }
}
