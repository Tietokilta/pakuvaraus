use crate::reservation::types::ReservationItem;
use crate::{error::AppResult, state::app_state::AppState};
use axum::extract::{Json, State};

pub async fn get_reservations(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<ReservationItem>>> {
    let reservations =
        crate::reservation::get_reservations_db::get_reservations(&state.db.pool).await?;

    Ok(Json(reservations.into_iter().map(Into::into).collect()))
}
