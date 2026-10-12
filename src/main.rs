use axum_login::{AuthManagerLayerBuilder, login_required};
use axum_messages::MessagesManagerLayer;
use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_sessions::cookie::SameSite;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tracing::info;

mod admin;
mod api;
mod auth;
mod contract;
mod db;
mod domain;
mod email;
mod error;
mod logging;
mod login;
mod mailgun;
mod reservation;
mod state;
mod user;
mod util;

#[tokio::main]
async fn main() {
    dotenv().ok();
    logging::tracing_filter::init();

    let test_password = "pass123";
    let test_plain = auth::types::PlainPassword(test_password.to_string());
    info!(
        "Test password: {}, hash: {}",
        test_password,
        auth::password::get_password_hash(test_plain)
            .await
            .unwrap()
            .as_str()
    );

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnSessionEnd);

    let app_state = state::app_state::new().await;
    let auth_layer = AuthManagerLayerBuilder::new(app_state.clone(), session_layer).build();

    let app = admin::routes::router()
        .route_layer(login_required!(
            state::app_state::AppState,
            login_url = "/login"
        ))
        .merge(api::routes::router())
        .with_state(app_state)
        .merge(login::routes::router())
        .nest_service("/static", ServeDir::new("static"))
        .layer(MessagesManagerLayer)
        .layer(auth_layer);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => {
            println!("Listen on {addr}");
            listener
        }
        Err(e) => {
            eprintln!("Listener bind failed on address {addr}: {e}");
            return;
        }
    };

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
