use askama::Template;
use axum::{
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use axum_login::AuthSession;

use crate::state::AppState;
use crate::logic::{get_reservations, get_admins};

#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate<'a> {
    reservations: Vec<String>,
    admins: Vec<String>,
    username: &'a str,
}

pub fn router() -> Router<()> {
    Router::new().route("/admin", get(self::get::protected))
}

mod get {
    use super::*;

    pub async fn protected(auth_session: AuthSession<AppState>) -> impl IntoResponse {
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
}