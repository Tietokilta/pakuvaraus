use axum::{
    extract::{Path},
    response::Redirect,
};

use crate::{auth::auth_session::AuthenticatedSession, error::{AppError, AppResult}};

pub async fn remove_reservation(auth_session: AuthenticatedSession, Path(id): Path<i32>) -> AppResult<Redirect> {
    crate::reservation::remove_reservation_db::remove_reservation(
        &auth_session.session.backend.db.pool,
        id,
    ).await?
     .ok_or(AppError::NotFound)?;

    Ok(Redirect::to("/admin"))
}
