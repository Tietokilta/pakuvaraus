use askama::Template;
use axum::{
    extract::{Form},
    response::{Html, IntoResponse, Redirect},
};

use crate::users::{SetUserPasswordData, AuthenticatedSession, set_user_password};

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

pub async fn change_password(auth_session: AuthenticatedSession, Form(payload): Form<SetUserPasswordData>) -> impl IntoResponse {
    set_user_password(&auth_session, &payload).await;
    Redirect::to("/admin").into_response()
}