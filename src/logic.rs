use sqlx::postgres::PgPool;

use crate::reservations::{Reservation, CreateReservation, ReservationValidation};

async fn validate_reservation(
    db_pool: &PgPool,
    reservation: &CreateReservation
) -> ReservationValidation {
    if reservation.end_time < reservation.start_time { 
        return ReservationValidation {is_valid: false, reason: String::from("Start time must be before end time")}; 
    }

    let overlapping: Vec<Reservation> = sqlx::query_as!(
        Reservation, 
            "SELECT id, reservation_name, start_time, end_time FROM reservations res WHERE res.start_time <= $2 AND res.end_time >= $1",
            reservation.start_time,
            reservation.end_time
        )
        .fetch_all(db_pool)
        .await
        .expect("Failed to fetch reservations");

    if overlapping.len() != 0 {
        return ReservationValidation {is_valid: false, reason: String::from("There is an overlapping reservation")};
    }

    ReservationValidation {is_valid: true, reason: String::from("Valid")}
}

pub async fn add_reservation(
    db_pool: &PgPool,
    reservation: &CreateReservation
) -> ReservationValidation {
    let valid= validate_reservation(db_pool, reservation).await;

    if valid.is_valid {
        sqlx::query_as!(
            Reservation,
            "INSERT INTO reservations (reservation_name, start_time, end_time) VALUES ($1, $2, $3) RETURNING id, reservation_name, start_time, end_time",
            reservation.reservation_name,
            reservation.start_time,
            reservation.end_time
        )
        .fetch_one(db_pool)
        .await
        .expect("Failed to create reservation");
    }

    valid
}

pub async fn get_reservations(db_pool: &PgPool) -> Vec<Reservation> {
    let reservations = sqlx::query_as!(Reservation, "SELECT id, reservation_name, start_time, end_time FROM reservations")
        .fetch_all(db_pool)
        .await
        .expect("Failed to fetch reservations");

    reservations
}