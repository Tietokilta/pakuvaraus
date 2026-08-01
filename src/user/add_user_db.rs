use sqlx::PgPool;
use tracing::info;

use super::types::User;
use crate::auth::types::PasswordHash;

pub async fn add_user(
    db_pool: &PgPool,
    username: &String,
    password: PasswordHash
) {
    info!("Try to create new user: {}", username);
    if username.len() != 0 {
        let res = sqlx::query_as!(
            User,
            "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id, username, password",
            username,
            password.to_string()
        )
        .fetch_one(db_pool)
        .await;

        match res {
            Ok(_) => (),
            Err(err) => {
                info!("Create user failed, database error: {}", err.to_string());
            }
        }
    } else {
        info!("Create user data not valid");
    }
}