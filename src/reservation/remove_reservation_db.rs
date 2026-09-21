use sqlx::PgPool;
use tracing::info;

use crate::db::{DbError, UpdateOutcome};

pub async fn remove_reservation(
    db_pool: &PgPool,
    id: i32,
) -> Result<UpdateOutcome, DbError> {
    info!("Try to remove reservation: {}", id);
    let res = sqlx::query!(
        "DELETE FROM reservations WHERE id = $1",
        id
    )
    .execute(db_pool)
    .await?;

    Ok(UpdateOutcome::from(res)).inspect(|o| match o {
        UpdateOutcome::Updated => info!("Reservation successfully deleted: {id}"),
        UpdateOutcome::NotFound => info!("Remove reservation failed, not found: {id}"),
    })
}
