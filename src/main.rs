use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_sessions::{Expiry, SessionManagerLayer, MemoryStore};
use tower_sessions::cookie::SameSite;
use axum_login::{AuthManagerLayerBuilder, login_required};
use axum_messages::{MessagesManagerLayer};
use tracing_subscriber;

mod admin;
mod handlers;
mod logic;
mod login;
mod reservations;
mod state;
mod users;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnSessionEnd);    
    
    let app_state = state::new().await;
    let auth_layer = AuthManagerLayerBuilder::new(app_state.clone(), session_layer).build();

    let app = admin::router()
        .route_layer(login_required!(state::AppState, login_url = "/login"))
        .merge(login::router())
        .merge(handlers::router(app_state))
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