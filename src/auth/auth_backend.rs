use axum_login::{AuthUser, AuthnBackend, UserId};
use tokio::task;
use tracing::info;

use super::{
    password::check_plain_password,
    types::Credentials,
};
use crate::state::app_state::AppState;
use crate::user::types::User;

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_str().as_bytes()
    }
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
        let user = sqlx::query_as!(
            Self::User,
            r#"SELECT id, username AS "username: _", password AS "password: _" FROM users WHERE username = $1"#,
            creds.username.as_str()
        ).fetch_optional(&self.db.pool)
         .await?;

        task::spawn_blocking(move || {
            Ok(user.filter(|user| check_plain_password(creds.password, &user.password)))
        }).await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        info!("Get user");
        let user = sqlx::query_as("select * from users where id = $1")
            .bind(user_id)
            .fetch_optional(&self.db.pool)
            .await?;

        Ok(user)
    }
}
