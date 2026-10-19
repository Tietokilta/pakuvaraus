use anyhow::Context;
use sqlx::PgPool;

use super::password::get_password_hash;
use super::types::NewPassword;
use crate::{
    domain::Username,
    error::AppResult,
    user::{add_user_db::add_user, types::User},
};

pub async fn register_user(
    db_pool: &PgPool,
    username: &Username,
    password: NewPassword,
) -> AppResult<User> {
    let hash = get_password_hash(password.into_plain())
        .await
        .context("panic in generating password hash")?;
    Ok(add_user(db_pool, username, hash).await?)
}
