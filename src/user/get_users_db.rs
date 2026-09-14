use sqlx::PgPool;

use super::types::UserDataObject;

pub async fn get_users(db_pool: &PgPool) -> Vec<UserDataObject> {
    sqlx::query_as!(UserDataObject, "SELECT id, username FROM users")
        .fetch_all(db_pool)
        .await
        .expect("Failed to fetch users")
}

