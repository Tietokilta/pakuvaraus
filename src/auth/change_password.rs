use sqlx::PgPool;

use crate::auth::password::CheckPasswordError;
use crate::db::{DbError, UpdateOutcome};
use crate::user::set_password_db::set_password;

use super::password::{check_plain_password, get_password_hash};
use super::types::{HashedPassword, NewPassword, PlainPassword};

use tracing::debug;

#[derive(Debug, thiserror::Error)]
pub enum ChangePasswordError {
    #[error("wrong password")]
    WrongPassword,
    #[error("user not found")]
    UserNotFound,
    #[error("stored password hash is malformed")]
    MalformedHash(#[source] password_auth::ParseError),
    #[error(transparent)]
    DbError(#[from] DbError),
    #[error(transparent)]
    TokioError(#[from] tokio::task::JoinError),
}

impl From<CheckPasswordError> for ChangePasswordError {
    fn from(value: CheckPasswordError) -> Self {
        match value {
            CheckPasswordError::WrongPassword => Self::WrongPassword,
            CheckPasswordError::MalformedHash(parse_error) => Self::MalformedHash(parse_error),
            CheckPasswordError::TokioError(join_error) => Self::TokioError(join_error),
        }
    }
}

pub async fn change_password(
    db_pool: &PgPool,
    user_id: i32,
    stored_password: HashedPassword,
    entered_password: PlainPassword,
    new_password: NewPassword,
) -> Result<(), ChangePasswordError> {
    check_plain_password(entered_password, stored_password).await?;

    let new_hash = get_password_hash(new_password.into_plain()).await?;

    if set_password(db_pool, user_id, new_hash).await? == UpdateOutcome::NotFound {
        return Err(ChangePasswordError::UserNotFound);
    }
    debug!("password succesfully changed for {user_id}");
    Ok(())
}
