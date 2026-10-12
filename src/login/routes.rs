use super::core::{get_login_page, login};
use super::logout::logout;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(login))
        .route("/login", get(get_login_page))
        .route("/logout", get(logout))
}
