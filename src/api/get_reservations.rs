use axum::{
    extract::{Json, State},
};
use crate::state::app_state::AppState;
use crate::reservation::types::ReservationItem;

pub async fn get_reservations(State(state): State<AppState>) -> Json<Vec<ReservationItem>> {
    let reservations = crate::reservation::get_reservations_db::get_reservations(&state.db.pool).await;
    Json(reservations.into_iter().map(Into::into).collect())
}