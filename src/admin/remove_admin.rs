use axum::{
    extract::{Path},
    response::{IntoResponse, Redirect},
};

use crate::users::{AuthenticatedSession, remove_user};

pub async fn remove_admin(auth_session: AuthenticatedSession, Path(username): Path<String>) -> impl IntoResponse {
    remove_user(&auth_session, &username).await;
    Redirect::to("/admin").into_response()
}