use axum::response::{IntoResponse, Redirect, Response};

pub struct LoginRedirect;

impl IntoResponse for LoginRedirect {
    fn into_response(self) -> Response {
        Redirect::to("/login").into_response()
    }
}
