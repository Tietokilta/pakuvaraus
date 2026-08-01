use sqlx::PgPool;
use tracing::info;

use crate::auth::types::PasswordHash;

pub async fn set_password(
    db_pool: &PgPool,
    user_id: i32,
    password: PasswordHash,
) {
    info!("Try to set user password");
    let res = sqlx::query!(
        "UPDATE users SET password = $1 WHERE id = $2",
        password.to_string(),
        user_id,
    )
    .fetch_one(db_pool)
    .await;

    match res {
        Ok(_) => {
            info!("Password set successfully for user: {}", user_id);                        
        },
        Err(err) => {
            info!("Set password failed, database error: {}", err.to_string());
        }
    }
}
