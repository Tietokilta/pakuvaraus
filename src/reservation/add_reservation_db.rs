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
            "INSERT INTO reservations (reservation_name, start_time, end_time) VALUES ($1, $2, $3) RETURNING id, reservation_name, start_time, end_time",
            reservation.reservation_name,
            reservation.start_time,
            reservation.end_time
        )
        .fetch_one(db_pool)
        .await;

        match res {
            Ok(_) => (),
            Err(err) => {
                match err.as_database_error() {
                    Some(db_err) => {
                        match db_err.constraint()  {
                            Some(_) => {
                                return ReservationValidation {is_valid: false, reason: String::from("There is an overlapping reservation")};  
                            }
                            None => ()
                        }
                    },
                    None => ()
                }
            }
        }
    }

    valid
}