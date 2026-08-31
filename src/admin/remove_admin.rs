use axum::{
    extract::{Path},
    response::{IntoResponse, Redirect},
};

use crate::auth::auth_session::AuthenticatedSession;

pub async fn remove_admin(auth_session: AuthenticatedSession, Path(user_id): Path<i32>) -> impl IntoResponse {
    crate::user::delete_user_db::remove_user(
        &auth_session.backend.db.pool,
        user_id,
    ).await;
    Redirect::to("/admin").into_response()
}