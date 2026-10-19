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
