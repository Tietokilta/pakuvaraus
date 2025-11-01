use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use crate::state::AppState;
use crate::reservations::{Reservation, CreateReservation, ReservationValidation};
use crate::logic::{add_reservation, get_reservations};

pub fn router(state: AppState) -> Router<()> {
    Router::new()
        .route("/reservations", post(handle_reservation))
        .route("/reservations", get(handle_get_reservations))
        .with_state(state)
}

pub async fn handle_reservation(
    State(state): State<AppState>,
    Json(payload): Json<CreateReservation>,
) -> Json<ReservationValidation> {
    let valid= add_reservation(&state.db_pool, &payload).await;
    Json(valid)
}

pub async fn handle_get_reservations(State(state): State<AppState>) -> Json<Vec<Reservation>> {
    let reservations = get_reservations(&state.db_pool).await;
    Json(reservations)
}