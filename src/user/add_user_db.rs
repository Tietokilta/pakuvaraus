use sqlx::PgPool;
use tracing::info;

use crate::auth::types::HashedPassword;
use crate::db::{DbError, ResultExt};
use crate::domain::Username;

pub async fn add_user(
    db_pool: &PgPool,
    username: &Username,
    password: HashedPassword,
) -> Result<(), DbError> {
    info!("Try to create new user: {username}");
    sqlx::query!(
        r#"INSERT INTO users (username, password) VALUES ($1, $2)"#,
        username.as_str(),
        password.as_str()
    )
    .execute(db_pool)
    .await
    .on_constraint("users_username_key", |_| {
        DbError::Constraint("username already taken")
    })
    .map(|_| ())
}
