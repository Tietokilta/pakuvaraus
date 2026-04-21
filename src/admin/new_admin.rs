use askama::Template;
use axum::{
    extract::{Form},
    response::{Html, IntoResponse, Redirect},
};

use crate::users::{CreateUserData, AuthenticatedSession, add_user};

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

pub async fn new_admin(auth_session: AuthenticatedSession, Form(payload): Form<CreateUserData>) -> impl IntoResponse {
    add_user(&auth_session, &payload).await;
    Redirect::to("/admin").into_response()
}