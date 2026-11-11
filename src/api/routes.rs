use super::{get_reservations, make_reservation};
use crate::state::app_state::AppState;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(OpenApi)]
#[openapi(info(
    title = "pakuvaraus",
    description = "Public API for requesting and cancelling van reservations."
))]
struct ApiDoc;

/// Public API routes together with their `OpenAPI` description.
pub fn router() -> OpenApiRouter<AppState> {
    let mut doc = ApiDoc::openapi();
    // Cargo.toml declares no license, which would otherwise show up as an empty one
    doc.info.license = None;
    OpenApiRouter::with_openapi(doc)
        .routes(routes!(
            make_reservation::make_reservation,
            get_reservations::get_reservations
        ))
        .routes(routes!(make_reservation::cancel_reservation))
}
