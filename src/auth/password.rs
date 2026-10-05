use password_auth::{VerifyError, generate_hash, verify_password};

use super::types::{HashedPassword, PlainPassword};

pub async fn get_password_hash(
    password: PlainPassword,
) -> Result<HashedPassword, tokio::task::JoinError> {
    tokio::task::spawn_blocking(move || HashedPassword(generate_hash(password.0))).await
}

#[derive(Debug, thiserror::Error)]
pub enum CheckPasswordError {
    #[error("wrong password")]
    WrongPassword,
    #[error("stored password hash is malformed")]
    MalformedHash(#[source] password_auth::ParseError),
    #[error("password check task failed")]
    TokioError(#[from] tokio::task::JoinError),
}

pub async fn check_plain_password(
    password: PlainPassword,
    hash: HashedPassword,
) -> Result<(), CheckPasswordError> {
    tokio::task::spawn_blocking(move || verify_password(password.0, hash.as_str()))
        .await?
        .map_err(|e| match e {
            VerifyError::Parse(parse_error) => CheckPasswordError::MalformedHash(parse_error),
            VerifyError::PasswordInvalid => CheckPasswordError::WrongPassword,
        })
}
