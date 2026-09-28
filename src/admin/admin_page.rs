use askama::Template;
use axum::{
    extract::Path,
    response::{Html, IntoResponse, Redirect},
};

use tracing::info;

use super::view::{UserView, OverlapView, ReservationView};
use crate::auth::auth_session::AuthenticatedSession;
use crate::reservation::{get_reservations_db::get_reservations, overlap::{find_overlaps, distribute_overlaps}, types::SortedReservations};
use crate::user::get_users_db::get_users;

#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate<'a> {
    reservations: Vec<ReservationView>,
    approved_reservations: Vec<ReservationView>,
    admins: Vec<UserView>,
    username: &'a str,
}

pub async fn admin_page(auth_session: AuthenticatedSession) -> impl IntoResponse {
    match &auth_session.user {
        Some(user) => {                
            let res =  SortedReservations::new(get_reservations(&auth_session.backend.db.pool).await);
            let admins = get_users(&auth_session.backend.db.pool).await;
            let overlaps = find_overlaps(&res);
            let res_overlaps = distribute_overlaps(&res, overlaps);

            let views: Vec<ReservationView> = res.iter().enumerate().map(|(i, r)| ReservationView::from_domain(
                r, &res_overlaps[i]
            )).collect();

            Html(
                AdminTemplate {
                    reservations: views.iter().filter(|r| !r.approved).cloned().collect(),
                    approved_reservations: views.iter().filter(|r| r.approved).cloned().collect(),
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

pub async fn approve_reservation(auth_session: AuthenticatedSession, Path(id): Path<i32>) -> impl IntoResponse {
    crate::reservation::approve_reservation_db::approve_reservation(
        &auth_session.backend.db.pool,
        true,
        id
    ).await;
    info!("Approve reservation: {id}");
    Redirect::to("/admin").into_response()
}