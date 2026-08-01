use sqlx::postgres::PgPool;
use std::env;
use dotenv::dotenv;
use axum_login::{AuthnBackend, UserId};
use tokio::task;
use tracing::info;

use crate::users::{Credentials, User, check_user_credentials};

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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

impl AuthnBackend for AppState {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = sqlx::query_as("select * from users where username = $1")
            .bind(&creds.username)
            .fetch_optional(&self.db_pool)
            .await?;
        
        task::spawn_blocking(move || {
            Ok(user.filter(|user| check_user_credentials(user, &creds)))
        })
        .await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        info!("Get user");
        let user = sqlx::query_as("select * from users where id = $1")
            .bind(user_id)
            .fetch_optional(&self.db_pool)
            .await?;

        Ok(user)
    }
}