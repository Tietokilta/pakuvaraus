use sqlx::PgPool;

use crate::db::DbError;

use super::types::{UserDataObject, User};

pub async fn get_users(db_pool: &PgPool) -> Result<Vec<UserDataObject>, DbError> {
    let res = sqlx::query_as!(UserDataObject, "SELECT id, username FROM users")
        .fetch_all(db_pool)
        .await?;
    Ok(res)
}

pub async fn get_one_user(db_pool: &PgPool, user_id: i32) -> Result<Option<User>, DbError> {
    let user = sqlx::query_as("select * from users where id = $1")
        .bind(user_id)
        .fetch_optional(db_pool)
        .await?;

    Ok(user)
}