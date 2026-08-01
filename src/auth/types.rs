use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub password: String,
    pub password_again: String,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub password: String,
    pub password_again: String,
}

pub struct PasswordHash(pub(crate) String);

impl PasswordHash {
    pub(crate) fn to_string(&self) -> &str {
        &self.0
    }
}

pub struct PlainPassword(pub String);