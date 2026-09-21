use sqlx::postgres::PgPool;

use crate::db::{DbError, ResultExt};
use crate::reservation::types::CreateReservation;

pub async fn add_reservation(
    db_pool: &PgPool,
    reservation: &CreateReservation,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"INSERT INTO reservations (full_name, email, start_time, end_time, internal) VALUES ($1, $2, $3, $4, $5)"#,
        reservation.full_name.as_str(),
        reservation.email.as_str(),
        reservation.time.start(),
        reservation.time.end(),
        reservation.internal,
    )
        .execute(db_pool)
        .await
        .on_constraint("block_overlapping_times", |_| DbError::Constraint("there is an overlapping reservation"))?;

    Ok(())
}
