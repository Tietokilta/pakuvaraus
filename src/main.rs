use dotenv::dotenv;
use pakuvaraus::auth::password::get_password_hash;
use pakuvaraus::logging;
use tracing::info;

use pakuvaraus::{auth::types::PlainPassword, state::{app_state::AppState, server::{Server, ServerConfig}}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    logging::tracing_filter::init();

    let test_password = "pass123";
    let test_plain = PlainPassword(test_password.to_string());
    info!(
        "Test password: {}, hash: {}",
        test_password,
        get_password_hash(test_plain).await.unwrap().as_str()
    );

    let config = ServerConfig::from_env()?;

    let app_state = AppState::new().await;
    app_state.db.migrate().await;
    let app = pakuvaraus::app(app_state);

    Server::from_config(&config).await?.run(app).await?;
    Ok(())
}
