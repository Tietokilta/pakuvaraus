use askama::Template;
use axum::{
    extract::{Form},
    response::{Html, Redirect},
};

use crate::{auth::types::RegisterUserRequest, error::AppResult};
use crate::auth::auth_session::AuthenticatedSession;

#[derive(Template)]
#[template(path = "new_admin.html")]
struct NewAdminTemplate {
}

pub async fn get_new_admin_page(_auth_session: AuthenticatedSession) -> AppResult<Html<String>> {
    Ok(Html(NewAdminTemplate {}.render()?))
}

pub async fn new_admin(auth_session: AuthenticatedSession, Form(payload): Form<RegisterUserRequest>) -> AppResult<Redirect> {
    crate::auth::register_user::register_user(
        &auth_session.session.backend.db.pool,
        &payload.username,
        payload.password,
    ).await?;
    Ok(Redirect::to("/admin"))
}
