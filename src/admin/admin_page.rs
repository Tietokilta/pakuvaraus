use askama::Template;
use axum::{
    response::{Html, IntoResponse, Redirect},
};

use super::view::{ReservationView, UserView};
use crate::auth::auth_session::AuthenticatedSession;
use crate::reservation::get_reservations_db::get_reservations;
use crate::user::get_users_db::get_users;

#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate<'a> {
    reservations: Vec<ReservationView>,
    admins: Vec<UserView>,
    username: &'a str,
}

pub async fn admin_page(auth_session: AuthenticatedSession) -> impl IntoResponse {
    match &auth_session.user {
        Some(user) => {                
            let res = get_reservations(&auth_session.backend.db.pool).await;
            let admins = get_users(&auth_session.backend.db.pool).await;

            Html(
                AdminTemplate {
                    //reservations: res.iter().map(|r| r.reservation_name.clone()).collect(),
                    //admins: admins.iter().map(|a| a.username.clone()).collect(),
                    reservations: res.iter().map(|r| ReservationView::from_domain(r.id, r.start_time, r.end_time, r.email.as_str())).collect(),
                    admins: admins.iter().map(|u| UserView::from_domain(u.id, u.username.clone())).collect(),
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