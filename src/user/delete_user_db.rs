use sqlx::PgPool;
use tracing::info;

pub async fn remove_user(
    db_pool: &PgPool,
    username: &String,
) {
    info!("Try to remove user");
    if username.len() != 0 {
        let res = sqlx::query!(
            "DELETE FROM users WHERE username = $1",
            username
        )
        .fetch_one(db_pool)
        .await;

        match res {
            Ok(_) => {
                info!("User successfully deleted: {}", username);                        
            },
            Err(err) => {
                info!("Remove user failed, database error: {}", err.to_string());
            }
        }
    } else {
        info!("Remove username not valid")
    }
}