use anyhow::anyhow;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Response},
};
use axum_login::AuthSession;
use tracing::info;

use crate::{
    auth::login_redirect::LoginRedirect, error::AppError, state::app_state::AppState,
    user::types::User,
};

pub struct AuthenticatedSession {
    pub session: AuthSession<AppState>,
    pub user: User,
}

impl<T> FromRequestParts<T> for AuthenticatedSession
where
    T: Send + Sync,
    AppState: FromRef<T>,
{
    type Rejection = Response;
    async fn from_request_parts(parts: &mut Parts, state: &T) -> Result<Self, Response> {
        let session: AuthSession<AppState> =
            AuthSession::<AppState>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    AppError::Anyhow(anyhow!("failed to build authsession from request"))
                        .into_response()
                })?;

        info!("Validate user");
        if let Some(user) = session.user.clone() {
            info!("Validated");
            return Ok(Self { session, user, });
        }
        info!("Validation failed");
        Err(LoginRedirect.into_response())
    }
}
