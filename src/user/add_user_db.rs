use sqlx::PgPool;
use tracing::info;

use super::types::User;
use crate::auth::types::HashedPassword;
use crate::domain::Username;

pub async fn add_user(
    db_pool: &PgPool,
    username: &Username,
    password: HashedPassword
) {
    info!("Try to create new user: {}", username.as_str());
    let res = sqlx::query_as!(
        User,
        r#"INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id, username AS "username: _", password AS "password: _""#,
        username.as_str(),
        password.as_str()
    )
    .fetch_one(db_pool)
    .await;

    match res {
        Ok(_) => (),
        Err(err) => {
            info!("Create user failed, database error: {}", err.to_string());
        }
    }
}
