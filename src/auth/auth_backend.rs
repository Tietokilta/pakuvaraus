use axum_login::{AuthUser, AuthnBackend, UserId};
use tracing::info;

use super::{password::check_plain_password, types::Credentials};
use crate::auth::password::CheckPasswordError;
use crate::error::AppError;
use crate::user::{get_users_db::{get_user_by_id, get_user_by_name}, types::User};
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
        let user = get_user_by_name(&self.db.pool, &creds.username.as_str()).await?;

        let Some(user) = user else {
            return Ok(None);
        };

        let password_valid = check_plain_password(creds.password, user.password.clone())
            .await
            .map(|()| user);

        match password_valid {
            Ok(user) => Ok(Some(user)),
            Err(CheckPasswordError::WrongPassword) => Ok(None),
            Err(
                err @ (CheckPasswordError::MalformedHash(_) | CheckPasswordError::TokioError(_)),
            ) => Err(anyhow::Error::from(err).into()),
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> AppResult<Option<Self::User>> {
        info!("Get user, id: {user_id}");
        Ok(get_user_by_id(&self.db.pool, *user_id).await?)
    }
}
