use axum::{
    response::{IntoResponse, Redirect},
    http::StatusCode,
};
use axum_login::AuthSession;

use crate::state::app_state::AppState;

pub async fn logout(mut auth_session: AuthSession<AppState>) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => Redirect::to("/login").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}