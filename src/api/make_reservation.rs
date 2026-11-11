use anyhow::Context;
use axum::extract::{Json, Path, State};

use tracing::{error, info};

use crate::{
    contract,
    email::email_templates::{
        ContractEmail, ReservationConfirmationEmail, ReservationEmail, send_contract_email,
        send_reservation_email,
    },
    error::{AppError, AppResult, ErrorResponse},
    reservation::{
        edit_reservation_db::set_cancel_by_key,
        types::{CreateReservation, EditKey},
    },
    state::app_state::AppState,
};

pub fn cancel_link(base: &str, key: &EditKey) -> String {
    format!(
        "{}/reservations/cancel/{}",
        base.trim_end_matches('/'),
        key.as_str()
    )
}

/// Request a reservation.
///
/// Generates the rental contract, emails it to the approver, and emails the
/// requester a confirmation with a cancel key. The reservation stays pending
/// until an admin approves it.
#[utoipa::path(
    post,
    path = "/reservations",
    tag = "reservations",
    request_body = CreateReservation,
    responses(
        (status = OK, description = "Reservation requested. The response body is empty."),
        (status = BAD_REQUEST, description = "The body is not valid JSON", body = String, content_type = "text/plain"),
        (status = UNPROCESSABLE_ENTITY, description = "A field is missing or invalid; the message names the problem", body = String, content_type = "text/plain"),
        (status = CONFLICT, description = "The time overlaps another internal reservation", body = ErrorResponse),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
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
    let edit_key = EditKey::new();
    crate::reservation::add_reservation_db::add_reservation(
        &state.db.pool,
        &payload,
        edit_key.hash(),
    )
    .await?;

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
    let _unhandled = send_reservation_email(
        &state.email,
        ReservationConfirmationEmail {
            reservation: &reservation_email,
            cancel_link: cancel_link("", &edit_key),
        },
    )
    .await
    .inspect_err(|err| error!(?err, "failed to send reservation email"));

    Ok(())
}

/// Cancel a reservation with the key from its confirmation email.
#[utoipa::path(
    post,
    path = "/reservations/cancel/{edit_key}",
    tag = "reservations",
    params(("edit_key" = String, Path, description = "Key from the confirmation email")),
    responses(
        (status = OK, description = "Reservation cancelled. The response body is empty."),
        (status = NOT_FOUND, description = "No reservation has this key", body = ErrorResponse),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
pub async fn cancel_reservation(
    State(state): State<AppState>,
    Path(raw_edit_key): Path<String>,
) -> AppResult<()> {
    info!("Cancel reservation");

    let edit_key = EditKey::parse(&raw_edit_key).ok_or(AppError::NotFound)?;

    if !set_cancel_by_key(&state.db.pool, &edit_key.hash(), true).await? {
        return Err(AppError::NotFound);
    }

    Ok(())
}
