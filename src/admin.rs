use askama::Template;
use axum::{
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use axum_login::AuthSession;

use crate::state::AppState;
use crate::logic::get_reservations;

#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate<'a> {
    reservations: Vec<String>,
    username: &'a str,
}

pub fn router() -> Router<()> {
    Router::new().route("/admin", get(self::get::protected))
}

mod get {
    use crate::reservations;

    use super::*;

    pub async fn protected(auth_session: AuthSession<AppState>) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => {
                //Html(format!("<h1>Admin page, user: {}</h1>", user.username)).into_response()
                
                let res = get_reservations(&auth_session.backend.db_pool).await;

                Html(
                    AdminTemplate {
                        reservations: res.iter().map(|r| r.reservation_name.clone()).collect(),
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