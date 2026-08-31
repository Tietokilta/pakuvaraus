use axum::{
    extract::{Path},
    response::{IntoResponse, Redirect},
};

use crate::auth::auth_session::AuthenticatedSession;

pub async fn remove_reservation(auth_session: AuthenticatedSession, Path(id): Path<i32>) -> impl IntoResponse {
    crate::reservation::remove_reservation_db::remove_reservation(
        &auth_session.backend.db.pool,
        id,
    ).await;
    Redirect::to("/admin").into_response()
}