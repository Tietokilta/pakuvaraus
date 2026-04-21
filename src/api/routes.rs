use axum::{
    routing::{get, post},
    Router,
};
use crate::state::AppState;
use super::handlers::{handle_reservation, handle_get_reservations};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/reservations", post(handle_reservation))
        .route("/reservations", get(handle_get_reservations))
}