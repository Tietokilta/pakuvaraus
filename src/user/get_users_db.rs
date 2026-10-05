use sqlx::PgPool;

use crate::db::DbError;

use super::types::{UserDataObject, User};

pub async fn get_users(db_pool: &PgPool) -> Result<Vec<UserDataObject>, DbError> {
    let res = sqlx::query_as!(UserDataObject, "SELECT id, username FROM users")
        .fetch_all(db_pool)
        .await?;
    Ok(res)
}

pub async fn get_user_by_id(db_pool: &PgPool, user_id: i32) -> Result<Option<User>, DbError> {
    let user = sqlx::query_as("select * from users where id = $1")
        .bind(user_id)
        .fetch_optional(db_pool)
        .await?;

    Ok(user)
}

pub async fn get_user_by_name(db_pool: &PgPool, username: &str) -> Result<Option<User>, DbError> {
    let user = sqlx::query_as("select * from users where username = $1")
        .bind(username)
        .fetch_optional(db_pool)
        .await?;

    Ok(user)
}