use super::ParseError;
use phonenumber::{Mode, PhoneNumber};
use serde::Deserialize;
use std::fmt;
use utoipa::openapi::{ObjectBuilder, RefOr, Schema, Type};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct Phone(PhoneNumber);
impl Phone {
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        // If no country code given, assume Finnish
        let n =
            phonenumber::parse(Some(phonenumber::country::FI), s).map_err(|_| ParseError::Phone)?;
        if !n.is_valid() {
            return Err(ParseError::Phone);
        }
        Ok(Self(n))
    }
}

impl utoipa::PartialSchema for Phone {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new()
            .schema_type(Type::String)
            .description(Some(
                "Phone number; a number without a country code is read as Finnish",
            ))
            .into()
    }
}
impl utoipa::ToSchema for Phone {}

impl TryFrom<String> for Phone {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

impl fmt::Display for Phone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format().mode(Mode::International))
    }
}
