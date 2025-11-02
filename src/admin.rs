use askama::Template;
use axum::{
    extract::Form,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use axum_login::AuthSession;

use crate::state::AppState;
use crate::logic::{get_reservations, get_admins};
use crate::users::{CreateUserData, add_user};

#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate<'a> {
    reservations: Vec<String>,
    admins: Vec<String>,
    username: &'a str,
}

#[derive(Template)]
#[template(path = "new_admin.html")]
struct NewAdminTemplate {

}

pub fn router() -> Router<()> {
    Router::new()
        .route("/admin", get(self::get::admin_page))
        .route("/admin/new", get(self::get::new_admin))
        .route("/admin/new", post(self::post::new_admin))
}

mod get {
    use super::*;

    pub async fn admin_page(auth_session: AuthSession<AppState>) -> impl IntoResponse {
        match &auth_session.user {
            Some(user) => {                
                let res = get_reservations(&auth_session.backend.db_pool).await;
                let admins = get_admins(&auth_session).await;

                Html(
                    AdminTemplate {
                        reservations: res.iter().map(|r| r.reservation_name.clone()).collect(),
                        admins: admins.iter().map(|a| a.username.clone()).collect(),
                        username: &user.username,
                    }
                    .render()
                    .unwrap(),
                    )
                .into_response()
            },
            None => Redirect::to("/login").into_response(),
        }
    }

    pub async fn new_admin(auth_session: AuthSession<AppState>) -> impl IntoResponse {
        match &auth_session.user {
            Some(_) => {
                Html(
                    NewAdminTemplate {

                    }
                    .render()
                    .unwrap(),
                    )
                .into_response()
            },
            None => Redirect::to("/login").into_response(),
        }
    }
}

mod post {
    use super::*;

    pub async fn new_admin(auth_session: AuthSession<AppState>, Form(payload): Form<CreateUserData>) -> impl IntoResponse {
        match &auth_session.user {
            Some(_) => {
                add_user(&auth_session, &payload).await;
                Redirect::to("/admin").into_response()
            },
            None => Redirect::to("/login").into_response(),
        }
    }
}