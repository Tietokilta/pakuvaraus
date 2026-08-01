use sqlx::PgPool;
use tracing::info;

pub async fn remove_user(
    db_pool: &PgPool,
    user_id: i32,
) {
    info!("Try to remove user: {}", user_id);
    let res = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(db_pool)
    .await;

    match res {
        Ok(_) => {
            info!("User successfully deleted: {}", user_id);                        
        },
        Err(err) => {
            info!("Remove user failed, database error: {}", err.to_string());
        }
    }
}