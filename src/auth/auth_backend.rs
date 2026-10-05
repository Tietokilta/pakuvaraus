use axum_login::{AuthUser, AuthnBackend, UserId};
use tracing::info;

use super::{password::check_plain_password, types::Credentials};
use crate::auth::password::CheckPasswordError;
use crate::error::AppError;
use crate::user::types::User;
use crate::{error::AppResult, state::app_state::AppState};

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

impl AuthnBackend for AppState {
    type User = User;
    type Credentials = Credentials;
    type Error = AppError;

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

        let Some(user) = user else {
            return Ok(None);
        };

        let result = check_plain_password(creds.password, user.password.clone())
            .await
            .map(|()| user);

        match result {
            Ok(user) => Ok(Some(user)),
            Err(CheckPasswordError::WrongPassword) => Ok(None),
            Err(
                err @ (CheckPasswordError::MalformedHash(_) | CheckPasswordError::TokioError(_)),
            ) => Err(anyhow::Error::from(err).into()),
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> AppResult<Option<Self::User>> {
        info!("Get user");
        let user = sqlx::query_as("select * from users where id = $1")
            .bind(user_id)
            .fetch_optional(&self.db.pool)
            .await?;

        Ok(user)
    }
}
