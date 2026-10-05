use sqlx::PgPool;

use crate::db::DbError;

use super::types::UserDataObject;

pub async fn get_users(db_pool: &PgPool) -> Result<Vec<UserDataObject>, DbError> {
    let res = sqlx::query_as!(UserDataObject, "SELECT id, username FROM users")
        .fetch_all(db_pool)
        .await?;
    Ok(res)
}
