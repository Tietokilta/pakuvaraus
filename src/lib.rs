use axum::Router;
use axum_login::{AuthManagerLayerBuilder, login_required};
use axum_messages::MessagesManagerLayer;
use tower_http::services::ServeDir;
use tower_sessions::cookie::SameSite;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

use crate::state::app_state::AppState;

mod admin;
mod api;
pub mod auth;
mod contract;
pub mod db;
pub mod domain;
pub mod email;
mod error;
pub mod logging;
mod login;
mod mailgun;
pub mod reservation;
pub mod state;
pub mod user;
mod util;

pub fn app(app_state: AppState) -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnSessionEnd);

    let auth_layer = AuthManagerLayerBuilder::new(app_state.clone(), session_layer).build();

    admin::routes::router()
        .route_layer(login_required!(
            state::app_state::AppState,
            login_url = "/login"
        ))
        .merge(api::routes::router())
        .with_state(app_state)
        .merge(login::routes::router())
        .nest_service("/static", ServeDir::new("static"))
        .layer(MessagesManagerLayer)
        .layer(auth_layer)
}
