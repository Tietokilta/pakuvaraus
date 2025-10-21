use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use handlers::{create_reservation, get_reservations};

mod handlers;
mod reservations;
mod state;

#[tokio::main]
async fn main() {
    let app_state = state::new().await;

    let app = Router::new()
        .route("/reservations", post(create_reservation))
        .route("/reservations", get(get_reservations))
        .with_state(app_state);

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