
use anyhow::Context;
use askama::Template;
use axum::{
    extract::Form,
    response::{Html, Redirect},
};
use axum_login::AuthSession;
use axum_messages::Messages;
use tracing::info;

use crate::auth::types::Credentials;
use crate::{
    error::{
        AppError::InvalidCredentials,
        AppResult,
    },
    state::app_state::AppState,
};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

pub async fn get_login_page() -> AppResult<Html<String>> {
    Ok(Html(LoginTemplate {}.render()?))
}

pub async fn login(
    mut auth_session: AuthSession<AppState>,
    messages: Messages,
    Form(creds): Form<Credentials>,
) -> AppResult<Redirect> {
    info!("Try to log in");
    let user = auth_session
        .authenticate(creds.clone())
        .await
        .with_context(|| format!("panic while authenticating {}", creds.username))?
        .ok_or(InvalidCredentials)?;

    auth_session
        .login(&user)
        .await
        .with_context(|| format!("login failed for {}", creds.username))?;

    messages.success(format!("Successfully logged in as {}", user.username));

    Ok(Redirect::to("/admin"))
}
