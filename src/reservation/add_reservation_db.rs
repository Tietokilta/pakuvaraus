use sqlx::postgres::PgPool;

use tracing::info;

use super::validate::validate_reservation;
use crate::reservation::types::{Reservation, CreateReservation, ReservationValidation};

pub async fn add_reservation(
    db_pool: &PgPool,
    reservation: &CreateReservation
) -> ReservationValidation {
    let valid= validate_reservation(reservation).await;

    if valid.is_valid {
        info!("Reservation is valid");
        let res = sqlx::query_as!(
            Reservation,
            "INSERT INTO reservations (reservation_name, email, start_time, end_time, internal, approved) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, reservation_name, email, start_time, end_time, internal, approved",
            reservation.reservation_name,
            reservation.email,
            reservation.start_time,
            reservation.end_time,
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
    }

    valid
}
