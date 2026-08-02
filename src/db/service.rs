use sqlx::postgres::PgPool;

use super::config::DbConfig;

#[derive(Clone)]
pub struct DbService {
    pub pool: PgPool,
}

impl DbService{
    pub async fn from_config(config: &DbConfig) -> Self {
        let db_pool = PgPool::connect(&config.url)
            .await
            .expect("DB pool create failed");

        Self {
            pool: db_pool,
        }
    }
}