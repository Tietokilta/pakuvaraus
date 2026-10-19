use super::get_reservations::get_reservations;
use super::make_reservation::{cancel_reservation, make_reservation};
use crate::state::app_state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/reservations", post(make_reservation))
        .route(
            "/reservations/cancel/{raw_edit_key}",
            post(cancel_reservation),
        )
        .route("/reservations", get(get_reservations))
}
