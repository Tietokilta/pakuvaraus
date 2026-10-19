use serde::{Deserialize, Serialize};

use crate::domain::{ParseError, Username};

#[derive(Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: Username,
    pub password: PlainPassword,
}

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub username: Username,
    // `password` and `password_again` in the form
    #[serde(flatten)]
    pub password: NewPassword,
}

#[derive(Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: PlainPassword,
    // `password` and `password_again` in the form
    #[serde(flatten)]
    pub new_password: NewPassword,
}

#[derive(Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct HashedPassword(pub(crate) String);

impl HashedPassword {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PlainPassword(pub String);

impl From<String> for PlainPassword {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(try_from = "RawNewPassword")]
pub struct NewPassword(PlainPassword);

#[derive(Deserialize)]
struct RawNewPassword {
    password: String,
    password_again: String,
}

impl NewPassword {
    pub const fn as_plain(&self) -> &PlainPassword {
        &self.0
    }

    pub fn into_plain(self) -> PlainPassword {
        self.0
    }
}

impl TryFrom<RawNewPassword> for NewPassword {
    type Error = ParseError;

    fn try_from(raw: RawNewPassword) -> Result<Self, Self::Error> {
        if raw.password.is_empty() {
            return Err(ParseError::EmptyPassword);
        }
        if raw.password != raw.password_again {
            return Err(ParseError::PasswordMismatch);
        }
        Ok(Self(PlainPassword(raw.password)))
    }
}
