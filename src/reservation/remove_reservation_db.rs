use sqlx::PgPool;
use tracing::info;

pub async fn remove_reservation(
    db_pool: &PgPool,
    id: i32,
) {
    info!("Try to remove reservation: {}", id);
    let res = sqlx::query!(
        "DELETE FROM reservations WHERE id = $1",
        id
    )
    .fetch_one(db_pool)
    .await;

    match res {
        Ok(_) => {
            info!("Reservation successfully deleted: {}", id);                        
        },
        Err(err) => {
            info!("Remove reservation failed, database error: {}", err.to_string());
        }
    }
}