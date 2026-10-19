use sqlx::PgPool;
use tracing::info;

use crate::auth::types::HashedPassword;
use crate::db::{DbError, ResultExt};
use crate::domain::Username;
use crate::user::types::User;

pub async fn add_user(
    db_pool: &PgPool,
    username: &Username,
    password: HashedPassword,
) -> Result<User, DbError> {
    info!("Try to create new user: {username}");
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password)
        VALUES ($1, $2)
        RETURNING id, username AS "username: _", password AS "password: _"
        "#,
        username.as_str(),
        password.as_str()
    )
    .fetch_one(db_pool)
    .await
    .on_constraint("users_username_key", |_| {
        DbError::Constraint("username already taken")
    })
}
