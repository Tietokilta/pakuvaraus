use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => {
            println!("Listening on {}", addr);
            listener
        },
        Err(e) => {
            eprintln!("Failed to bind to address {}: {}", addr, e);
            return;
        }
    };
    
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!"
}