use axum::{
    extract::{Json, State},
};
use crate::state::app_state::AppState;
use crate::reservations::{CreateReservation, ReservationValidation};
use crate::logic::{add_reservation};
use tracing::info;

pub async fn make_reservation(
    State(state): State<AppState>,
    Json(payload): Json<CreateReservation>,
) -> Json<ReservationValidation> {
    info!("Make reservation");
    let valid= add_reservation(&state.db_pool, &payload).await;
    Json(valid)
}