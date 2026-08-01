use axum::{
    extract::{Json, State},
};
use crate::state::app_state::AppState;
use crate::reservation::types::Reservation;

pub async fn get_reservations(State(state): State<AppState>) -> Json<Vec<Reservation>> {
    let reservations = crate::reservation::get_reservations_db::get_reservations(&state.db_pool).await;
    Json(reservations)
}