use anyhow::Context;
use axum::extract::{Json, State};

use tracing::{error, info};

use crate::{
    contract,
    email::email_templates::{
        ContractEmail, ReservationEmail, send_contract_email, send_reservation_email,
    },
    error::AppResult,
    reservation::types::CreateReservation,
    state::app_state::AppState,
};

pub async fn make_reservation(
    State(state): State<AppState>,
    Json(payload): Json<CreateReservation>,
) -> AppResult<()> {
    info!("Make reservation");

    let contract_bytes = tokio::task::spawn_blocking({
        let contract_inputs = payload.clone().into();
        move || contract::generate_pdf(contract_inputs)
    })
    .await
    .context("tokio thread panic while generating pdf")?
    .context("failed to generate contract pdf")?;

    // TODO: what if adding reservation works, but email fails?
    crate::reservation::add_reservation_db::add_reservation(&state.db.pool, &payload).await?;

    let reservation_email: ReservationEmail<'_> = (&payload).into();
    let _unhandled = send_contract_email(
        &state.email,
        ContractEmail {
            reservation: &reservation_email,
            contract_bytes,
        },
    )
    .await
    .inspect_err(|err| error!(?err, "failed to send contract email"));

    // Reservation already created, so don't return an error if email send fails
    let _unhandled = send_reservation_email(&state.email, &reservation_email)
        .await
        .inspect_err(|err| error!(?err, "failed to send reservation email"));
    Ok(())
}
