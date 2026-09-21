use super::{NonEmptyText, ParseError};
use serde::Deserialize;

macro_rules! nonemptytext_newtype {
    ($(
        $(#[$meta:meta])*
        $name:ident($max:literal) $(validate_with $validate:expr)?;
    )+) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Deserialize, PartialEq, Eq, sqlx::Type)]
            #[sqlx(transparent)]
            #[serde(try_from = "String")]
            pub struct $name(NonEmptyText<$max>);

            impl $name {
                pub fn as_str(&self) -> &str {
                    self.0.as_str()
                }
            }
            impl TryFrom<String> for $name {
                type Error = ParseError;

                fn try_from(value: String) -> Result<Self, Self::Error> {
                    let text = value.trim().to_owned().try_into()?;
                    $(
                        let validate: fn(&str) -> Result<(), ParseError> = $validate;
                        validate(text.as_str())?;
                    )?
                    Ok(Self(text))
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::Display::fmt(&self.0, f)
                }
            }
        )+
    }
}

nonemptytext_newtype!(
    Username(64);
    FullName(512);
    Address(128);
);
