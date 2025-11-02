use axum::{
    response::{Html, IntoResponse, Redirect},
    extract::Form,
    routing::{get, post},
    Router,
    http::StatusCode,
};
use axum_login::AuthSession;
use axum_messages::Messages;
use tracing::info;

use crate::state::AppState;
use crate::users::Credentials;

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(self::post::login))
        .route("/login", get(self::get::login))
        .route("/logout", get(self::get::logout))
}

mod post {
    use super::*;

    pub async fn login(
        mut auth_session: AuthSession<AppState>,
        messages: Messages,
        Form(creds): Form<Credentials>,
    ) -> impl IntoResponse {
        info!("Try to log in");
        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                messages.error("Invalid credentials");

                return Redirect::to("/login").into_response();
            }
            Err(_) => {
                info!("Error");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        };

        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        messages.success(format!("Successfully logged in as {}", user.username));

        Redirect::to("/admin").into_response()
    }
}

mod get {
    use super::*;

    pub async fn login() -> impl IntoResponse {
        Html(r#"
            <!doctype html>
            <html>
                <head><title>Login</title></head>
                <body>
                    <h1>Login</h1>
                    <form action="/login" method="post">
                        <label for="username">Username:</label>
                        <input type="text" id="username" name="username"><br><br>
                        <label for="password">Password:</label>
                        <input type="password" id="password" name="password"><br><br>
                        <input type="submit" value="Login">
                    </form>
                </body>
            </html>
        "#)
    }

    pub async fn logout(mut auth_session: AuthSession<AppState>) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => Redirect::to("/login").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}