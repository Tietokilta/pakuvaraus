use axum::{
    extract::{Json, State},
};
use crate::state::AppState;
use crate::reservations::{Reservation, CreateReservation};

pub async fn create_reservation(
    State(state): State<AppState>,
    Json(payload): Json<CreateReservation>,
) -> Json<Reservation> {
    let new_reservation = sqlx::query_as!(
        Reservation,
        "INSERT INTO reservations (reservation_name) VALUES ($1) RETURNING id, reservation_name",
        payload.reservation_name
    )
    .fetch_one(&state.db_pool)
    .await
    .expect("Failed to create reservation");

    Json(new_reservation)
}

pub async fn get_reservations(State(state): State<AppState>) -> Json<Vec<Reservation>> {
    let reservations = sqlx::query_as!(Reservation, "SELECT id, reservation_name FROM reservations")
        .fetch_all(&state.db_pool)
        .await
        .expect("Failed to fetch reservations");

    Json(reservations)
}