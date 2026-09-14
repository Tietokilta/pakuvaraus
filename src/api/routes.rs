use axum::{
    routing::{get, post},
    Router,
};
use crate::state::app_state::AppState;
use super::get_reservations::get_reservations;
use super::make_reservation::make_reservation;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/reservations", post(make_reservation))
        .route("/reservations", get(get_reservations))
}
