use sqlx::postgres::PgPool;

use crate::db::{DbError, ResultExt};
use crate::reservation::types::{CreateReservation, Reservation};

pub async fn add_reservation(
    db_pool: &PgPool,
    reservation: &CreateReservation,
) -> Result<Reservation, DbError> {
    sqlx::query_as!(
        Reservation,
        r#"
        INSERT INTO reservations (full_name, email, start_time, end_time, internal)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
            id, full_name AS "full_name: _", email AS "email: _",
            start_time, end_time, internal, approved
        "#,
        reservation.full_name.as_str(),
        reservation.email.as_str(),
        reservation.time.start(),
        reservation.time.end(),
        reservation.internal,
    )
    .fetch_one(db_pool)
    .await
    .on_constraint("block_overlapping_times", |_| {
        DbError::Constraint("there is an overlapping reservation")
    })
}
