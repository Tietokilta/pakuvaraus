use axum::extract::{Json, State};

use tracing::{error, info};

use crate::reservation::types::CreateReservation;
use crate::state::app_state::AppState;
use crate::{email::email_templates::send_reservation_email, error::AppResult};

pub async fn make_reservation(
    State(state): State<AppState>,
    Json(payload): Json<CreateReservation>,
) -> AppResult<()> {
    info!("Make reservation");

    // TODO: what if adding reservation works, but email fails?
    crate::reservation::add_reservation_db::add_reservation(&state.db.pool, &payload).await?;

    // Reservation already created, so don't return an error if email send fails
    let _unhandled = send_reservation_email(&state.email, payload.email.as_str())
        .await
        .inspect_err(|err| error!(?err, "failed to send reservation email"));
    Ok(())
}
