use dotenv::dotenv;
use pakuvaraus::auth::password::get_password_hash;
use pakuvaraus::logging;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

use pakuvaraus::auth::types::PlainPassword;

#[tokio::main]
async fn main() {
    dotenv().ok();
    logging::tracing_filter::init();

    let test_password = "pass123";
    let test_plain = PlainPassword(test_password.to_string());
    info!(
        "Test password: {}, hash: {}",
        test_password,
        get_password_hash(test_plain).await.unwrap().as_str()
    );

    let app_state = pakuvaraus::state::app_state::AppState::new().await;
    let app = pakuvaraus::app(app_state);

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string());
    let bind_port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let socket_addr: SocketAddr = format!("{bind_addr}:{bind_port}")
        .parse()
        .expect("BIND_ADDR env var must be an IP address");

    let listener = match TcpListener::bind(socket_addr).await {
        Ok(listener) => {
            println!("Listen on {socket_addr}");
            listener
        }
        Err(e) => {
            eprintln!("Listener bind failed on address {socket_addr}: {e}");
            return;
        }
    };

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
