use axum::{
    routing::{get, post},
    Router,
};
use crate::state::AppState;
use super::get_reservations::get_reservations;
use super::make_reservation::make_reservation;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/reservations", post(get_reservations))
        .route("/reservations", get(make_reservation))
}