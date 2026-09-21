use anyhow::Context;
use axum::response::Redirect;
use axum_login::AuthSession;

use crate::{error::AppResult, state::app_state::AppState};

pub async fn logout(mut auth_session: AuthSession<AppState>) -> AppResult<Redirect> {
    auth_session.logout().await.with_context(|| {
        format!(
            "panic while logging out {}",
            auth_session
                .user
                .map_or_else(|| "[missing user]".to_owned(), |u| u.username.into())
        )
    })?;

    Ok(Redirect::to("/login"))
}
