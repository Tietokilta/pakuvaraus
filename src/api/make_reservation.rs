use axum::{
    extract::{Json, State},
};
use crate::state::AppState;
use crate::reservations::{CreateReservation, ReservationValidation};
use crate::logic::{add_reservation};

pub async fn make_reservation(
    State(state): State<AppState>,
    Json(payload): Json<CreateReservation>,
) -> Json<ReservationValidation> {
    let valid= add_reservation(&state.db_pool, &payload).await;
    Json(valid)
}