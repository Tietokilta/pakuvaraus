use askama::Template;
use axum::{
    extract::{Form},
    response::{Html, IntoResponse, Redirect},
};

use crate::auth::types::{ChangePasswordRequest, PlainPassword, PasswordHash};
use crate::auth::auth_session::AuthenticatedSession;

use tracing::info;

#[derive(Template)]
#[template(path = "change_password.html")]
struct ChangePasswordTemplate {

}

pub async fn get_change_password_page(_auth_session: AuthenticatedSession) -> impl IntoResponse {
    Html(
        ChangePasswordTemplate {

        }
        .render()
        .unwrap(),
        )
    .into_response()
}

pub async fn change_password(auth_session: AuthenticatedSession, Form(payload): Form<ChangePasswordRequest>) -> impl IntoResponse {
    match &auth_session.user {
        Some(user) => {
            info!("Change password for user {}", user.username);
            crate::auth::change_password::change_password(
                &auth_session.backend.db.pool,
                user.id,
                PasswordHash(user.password.clone()),
                PlainPassword(payload.old_password),
                PlainPassword(payload.password),
                PlainPassword(payload.password_again)
            ).await;
            return Redirect::to("/admin").into_response();
        },
        None => {
            return Redirect::to("/login").into_response();
        },
    }
}