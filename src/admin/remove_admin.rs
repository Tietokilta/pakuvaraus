use axum::{extract::Path, response::Redirect};

use crate::{
    auth::auth_session::AuthenticatedSession,
    error::{AppError, AppResult},
};

pub async fn remove_admin(
    auth_session: AuthenticatedSession,
    Path(user_id): Path<i32>,
) -> AppResult<Redirect> {
    crate::user::delete_user_db::remove_user(&auth_session.session.backend.db.pool, user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Redirect::to("/admin"))
}
