use axum::{extract::Path, response::Redirect};

use crate::{
    auth::auth_session::AuthenticatedSession,
    error::{AppError, AppResult},
    reservation::edit_reservation_db::set_cancel_by_id,
};

pub async fn remove_reservation(
    auth_session: AuthenticatedSession,
    Path(id): Path<i32>,
) -> AppResult<Redirect> {
    crate::reservation::remove_reservation_db::remove_reservation(
        &auth_session.session.backend.db.pool,
        id,
    )
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Redirect::to("/admin"))
}

pub async fn cancel_reservation(
    auth_session: AuthenticatedSession,
    Path(id): Path<i32>,
) -> AppResult<Redirect> {
    if !set_cancel_by_id(&auth_session.session.backend.db.pool, id, true).await? {
        return Err(AppError::NotFound);
    }

    Ok(Redirect::to("/admin"))
}

pub async fn restore_reservation(
    auth_session: AuthenticatedSession,
    Path(id): Path<i32>,
) -> AppResult<Redirect> {
    if !set_cancel_by_id(&auth_session.session.backend.db.pool, id, false).await? {
        return Err(AppError::NotFound);
    }

    Ok(Redirect::to("/admin"))
}