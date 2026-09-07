use askama::Template;
use axum::{
    response::{Html, IntoResponse, Redirect},
    extract::Form,
    http::StatusCode,
};
use axum_login::AuthSession;
use axum_messages::Messages;
use tracing::info;

use crate::state::app_state::AppState;
use crate::auth::types::Credentials;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {

}

pub async fn get_login_page() -> impl IntoResponse {
    Html(
        LoginTemplate {

        }
        .render()
        .unwrap(),
        )
    .into_response()
}

pub async fn login(
    mut auth_session: AuthSession<AppState>,
    messages: Messages,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    info!("Try to log in");
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            messages.error("Invalid credentials");

            return Redirect::to("/login").into_response();
        }
        Err(e) => {
            info!("Error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    messages.success(format!("Successfully logged in as {}", user.username));

    Redirect::to("/admin").into_response()
}