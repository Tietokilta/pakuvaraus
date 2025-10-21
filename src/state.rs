use sqlx::postgres::PgPool;
use std::env;
use dotenv::dotenv;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

pub async fn new() -> AppState {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env not set");

    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("DB pool create failed");

    AppState { db_pool }
}