use sqlx::PgPool;
use tracing::info;

use crate::db::{DbError, UpdateOutcome};

pub async fn remove_user(db_pool: &PgPool, user_id: i32) -> Result<UpdateOutcome, DbError> {
    info!("Try to remove user: {}", user_id);
    let res = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(db_pool)
        .await?;

    Ok(UpdateOutcome::from(res)).inspect(|o| match o {
        UpdateOutcome::Updated => info!("User successfully deleted: {user_id}"),
        UpdateOutcome::NotFound => info!("User not found for deletion: {user_id}"),
    })
}
