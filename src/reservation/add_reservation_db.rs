use sqlx::postgres::PgPool;

use crate::reservation::types::{Reservation, CreateReservation, ReservationValidation};

pub async fn add_reservation(
    db_pool: &PgPool,
    reservation: &CreateReservation
) -> ReservationValidation {
    let res = sqlx::query_as!(
        Reservation,
        r#"INSERT INTO reservations (full_name, email, start_time, end_time, internal, approved) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, full_name AS "full_name: _", email AS "email: _", start_time, end_time, internal, approved"#,
        reservation.full_name.as_str(),
        reservation.email.as_str(),
        reservation.time.start(),
        reservation.time.end(),
        reservation.internal,
        false
    )
        .fetch_one(db_pool)
        .await;

    if let Err(err) = res
    && let Some(db_err) = err.as_database_error()
    && let Some(_) = db_err.constraint() {
        return ReservationValidation {is_valid: false, reason: String::from("There is an overlapping reservation")};
    }
    ReservationValidation {is_valid: true, reason: "is valid".to_string()}
}
