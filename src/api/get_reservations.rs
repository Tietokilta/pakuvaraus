use crate::reservation::types::ReservationItem;
use crate::{error::AppResult, state::app_state::AppState};
use axum::extract::{Json, State};

/// List the time slots of all reservations.
#[utoipa::path(
    get,
    path = "/reservations",
    tag = "reservations",
    responses(
        (status = OK, description = "Every reservation, including pending and cancelled ones", body = [ReservationItem]),
        (status = INTERNAL_SERVER_ERROR, body = crate::error::ErrorResponse),
    )
)]
pub async fn get_reservations(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<ReservationItem>>> {
    let reservations =
        crate::reservation::get_reservations_db::get_reservations(&state.db.pool).await?;

    Ok(Json(reservations.into_iter().map(Into::into).collect()))
}
