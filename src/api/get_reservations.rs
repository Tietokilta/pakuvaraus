use axum::{
    extract::{Json, State},
};
use crate::state::AppState;
use crate::reservations::{Reservation};

pub async fn get_reservations(State(state): State<AppState>) -> Json<Vec<Reservation>> {
    let reservations = crate::logic::get_reservations(&state.db_pool).await;
    Json(reservations)
}