use axum::{extract::{FromRequestParts, FromRef}, http::{request::Parts}};
use axum_login::AuthSession;
use tracing::info;
use std::ops::{Deref, DerefMut};

use super::login_redirect::LoginRedirect;
use crate::state::app_state::AppState;

pub struct AuthenticatedSession(pub AuthSession<AppState>);

impl Deref for AuthenticatedSession {
    type Target = AuthSession<AppState>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AuthenticatedSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> FromRequestParts<T> for AuthenticatedSession where T: Send + Sync, AppState: FromRef<T> {
    type Rejection = LoginRedirect;
    async fn from_request_parts(parts: &mut Parts, state: &T) -> Result<Self, Self::Rejection> {
        let session = AuthSession::<AppState>::from_request_parts(parts, state).await.map_err(|_|LoginRedirect)?;
        info!("Validate user");
        if let Some(_user) = &session.user {
            info!("Validated");
            return Ok(Self(session));
        }
        info!("Validation failed");
        Err(LoginRedirect)
    }
}
