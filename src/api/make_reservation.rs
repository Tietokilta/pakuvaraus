use axum::{
    extract::{Json, State},
};

use tracing::info;

use crate::email::email_templates::send_reservation_email;
use crate::state::app_state::AppState;
use crate::reservation::types::{CreateReservation, ReservationValidation};

pub async fn make_reservation(
    State(state): State<AppState>,
    Json(payload): Json<CreateReservation>,
) -> Json<ReservationValidation> {
    info!("Make reservation");
    let valid= crate::reservation::add_reservation_db::add_reservation(&state.db.pool, &payload).await;
    if valid.is_valid {
        send_reservation_email(&state.email, payload.email.as_str()).await;
    }
    Json(valid)
}
