use sqlx::postgres::PgPool;

use crate::db::{DbError, ResultExt};
use crate::reservation::types::{CreateReservation, Reservation, EditKeyHash};

pub async fn add_reservation(
    db_pool: &PgPool,
    reservation: &CreateReservation,
    edit_key: EditKeyHash,
) -> Result<Reservation, DbError> {
    sqlx::query_as!(
        Reservation,
        r#"
        INSERT INTO reservations (full_name, email, start_time, end_time, internal, edit_key)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
            id, full_name AS "full_name: _", email AS "email: _",
            start_time, end_time, internal, approved, cancelled
        "#,
        reservation.full_name.as_str(),
        reservation.email.as_str(),
        reservation.time.start(),
        reservation.time.end(),
        reservation.internal,
        edit_key.as_bytes().as_slice(),
    )
    .fetch_one(db_pool)
    .await
    .on_constraint("block_overlapping_times", |_| {
        DbError::Constraint("there is an overlapping reservation")
    })
}
