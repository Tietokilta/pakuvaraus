use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_sessions::{Expiry, SessionManagerLayer, MemoryStore};
use tower_sessions::cookie::SameSite;
use axum_login::{AuthManagerLayerBuilder, login_required};
use axum_messages::{MessagesManagerLayer};
use tracing::info;
use tracing_subscriber;

mod admin;
mod api;
mod auth;
mod logic;
mod login;
mod reservations;
mod state;
mod user;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let test_password = "pass123";
    //info!("Test password: {}, hash: {}", test_password, users::password_hash(&test_password.to_string()));

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnSessionEnd);    
    
    let app_state = state::app_state::new().await;
    let auth_layer = AuthManagerLayerBuilder::new(app_state.clone(), session_layer).build();

    let app = admin::routes::router()
        .route_layer(login_required!(state::app_state::AppState, login_url = "/login"))
        .merge(api::routes::router())
        .with_state(app_state)
        .merge(login::routes::router())
        .layer(MessagesManagerLayer)
        .layer(auth_layer);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => {
            println!("Listen on {}", addr);
            listener
        },
        Err(e) => {
            eprintln!("Listener bind failed on address {}: {}", addr, e);
            return;
        }
    };
    
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}