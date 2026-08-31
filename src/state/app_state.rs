//use sqlx::postgres::PgPool;
//use std::env;
use dotenv::dotenv;
use std::sync::Arc;

use super::app_config::AppConfig;
use crate::db::service::DbService;
use crate::email::service::EmailService;
use crate::mailgun::service::MailgunService;

#[derive(Clone)]
pub struct AppState {
    //pub db_pool: PgPool,
    pub db: DbService,
    //pub mailgun: MailgunService,
    pub email: Arc<dyn EmailService>,
}

pub async fn new() -> AppState {
    dotenv().ok();

    let config = AppConfig::from_env();

    AppState {
        db: DbService::create().await,
        email: Arc::new(MailgunService::from_config(&config.mailgun).await),
    }
}

/*
pub async fn new() -> AppState {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env not set");

    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("DB pool create failed");

    AppState { db_pool }
}
*/