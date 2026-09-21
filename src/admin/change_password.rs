use askama::Template;
use axum::{
    extract::Form,
    response::{Html, Redirect},
};

use crate::auth::{auth_session::AuthenticatedSession, change_password::ChangePasswordError};
use crate::{auth::types::ChangePasswordRequest, error::AppResult};

use tracing::info;

#[derive(Template)]
#[template(path = "change_password.html")]
struct ChangePasswordTemplate {}

pub async fn get_change_password_page(
    _auth_session: AuthenticatedSession,
) -> AppResult<Html<String>> {
    Ok(Html(ChangePasswordTemplate {}.render()?))
}

pub async fn change_password(
    auth_session: AuthenticatedSession,
    Form(payload): Form<ChangePasswordRequest>,
) -> AppResult<Redirect> {
    let user = auth_session.user;
    info!("Change password for user {}", user.username);
    let res = crate::auth::change_password::change_password(
        &auth_session.session.backend.db.pool,
        user.id,
        user.password.clone(),
        payload.old_password,
        payload.new_password,
    )
    .await;

    match res {
        Ok(()) => Ok(Redirect::to("/admin")),
        // TODO: return something that lets the user know the password was wrong
        Err(ChangePasswordError::WrongPassword) => Ok(Redirect::to("/admin/password")),
        Err(err) => Err(err.into()),
    }
}
