use sqlx::postgres::PgPool;

use crate::db::{DbError, UpdateOutcome};

pub async fn approve_reservation(
    db_pool: &PgPool,
    state: bool,
    id: i32,
) -> Result<UpdateOutcome, DbError> {
    let res = sqlx::query!(
        "UPDATE reservations SET approved = $1 WHERE id = $2",
        state,
        id
    )
    .execute(db_pool)
    .await?;

    Ok(UpdateOutcome::from(res))
}
