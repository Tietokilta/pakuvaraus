use askama::Template;
use axum::{
    extract::{Form, Path},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use axum_login::AuthSession;

use crate::state::AppState;
use crate::logic::{get_reservations, get_admins};
use crate::users::{CreateUserData, SetUserPasswordData, add_user, set_user_password, remove_user};

#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate<'a> {
    reservations: Vec<String>,
    admins: Vec<String>,
    username: &'a str,
}

#[derive(Template)]
#[template(path = "new_admin.html")]
struct NewAdminTemplate {

}

#[derive(Template)]
#[template(path = "change_password.html")]
struct ChangePasswordTemplate {

}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admin", get(self::get::admin_page))
        .route("/admin/{username}", get(self::get::remove_admin))
        .route("/admin/new", get(self::get::new_admin))
        .route("/admin/new", post(self::post::new_admin))
        .route("/admin/password", get(self::get::change_password))
        .route("/admin/password", post(self::post::change_password))
}

mod get {
    use crate::users::AuthenticatedUser;

    use super::*;

    pub async fn admin_page(auth_session: AuthSession<AppState>) -> impl IntoResponse {
        match &auth_session.user {
            Some(user) => {                
                let res = get_reservations(&auth_session.backend.db_pool).await;
                let admins = get_admins(&auth_session).await;

                Html(
                    AdminTemplate {
                        reservations: res.iter().map(|r| r.reservation_name.clone()).collect(),
                        admins: admins.iter().map(|a| a.username.clone()).collect(),
                        username: &user.username,
                    }
                    .render()
                    .unwrap(),
                    )
                .into_response()
            },
            None => Redirect::to("/login").into_response(),
        }
    }

    pub async fn new_admin(_auth_session: AuthenticatedUser) -> impl IntoResponse {
        return Html(
                    NewAdminTemplate {

                    }
                    .render()
                    .unwrap(),
                    )
                .into_response();
    }

    pub async fn change_password(auth_session: AuthSession<AppState>) -> impl IntoResponse {
        match &auth_session.user {
            Some(_) => {
                Html(
                    ChangePasswordTemplate {

                    }
                    .render()
                    .unwrap(),
                    )
                .into_response()
            },
            None => Redirect::to("/login").into_response(),
        }
    }
    
    pub async fn remove_admin(auth_session: AuthSession<AppState>, Path(username): Path<String>) -> impl IntoResponse {
        match &auth_session.user {
            Some(_) => {
                remove_user(&auth_session, &username).await;
                Redirect::to("/admin").into_response()
            },
            None => Redirect::to("/login").into_response(),
        }
    }
}

mod post {
    use super::*;

    pub async fn new_admin(auth_session: AuthSession<AppState>, Form(payload): Form<CreateUserData>) -> impl IntoResponse {
        match &auth_session.user {
            Some(_) => {
                add_user(&auth_session, &payload).await;
                Redirect::to("/admin").into_response()
            },
            None => Redirect::to("/login").into_response(),
        }
    }

    pub async fn change_password(auth_session: AuthSession<AppState>, Form(payload): Form<SetUserPasswordData>) -> impl IntoResponse {
        match &auth_session.user {
            Some(_) => {
                set_user_password(&auth_session, &payload).await;
                Redirect::to("/admin").into_response()
            },
            None => Redirect::to("/login").into_response(),
        }
    }
}