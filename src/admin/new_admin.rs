use askama::Template;
use axum::{
    extract::{Form},
    response::{Html, IntoResponse, Redirect},
};

use crate::auth::types::RegisterUserRequest;
use crate::auth::auth_session::AuthenticatedSession;

#[derive(Template)]
#[template(path = "new_admin.html")]
struct NewAdminTemplate {
}

pub async fn get_new_admin_page(_auth_session: AuthenticatedSession) -> impl IntoResponse {
    Html(
        NewAdminTemplate {

        }
        .render()
        .unwrap(),
        )
    .into_response()
}

pub async fn new_admin(auth_session: AuthenticatedSession, Form(payload): Form<RegisterUserRequest>) -> impl IntoResponse {
    crate::auth::register_user::register_user(
        &auth_session.backend.db.pool,
        &payload.username,
        &payload.password,
    ).await;
    Redirect::to("/admin").into_response()
}
