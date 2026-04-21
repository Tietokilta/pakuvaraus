use axum::{
    routing::{get, post},
    Router,
};
use super::login::{get_login_page, login};
use super::logout::logout;

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(login))
        .route("/login", get(get_login_page))
        .route("/logout", get(logout))
}