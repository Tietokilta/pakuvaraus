use askama::Template;
use axum::{
    extract::Path,
    response::{Html, Redirect},
};

use tracing::info;

use super::view::{ReservationView, UserView};
use crate::reservation::{
    get_reservations_db::get_reservations,
    overlap::{distribute_overlaps, find_overlaps},
    types::SortedReservations,
};
use crate::user::get_users_db::get_users;
use crate::{
    auth::auth_session::AuthenticatedSession,
    error::{AppError, AppResult},
};

#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate<'a> {
    reservations: Vec<ReservationView>,
    approved_reservations: Vec<ReservationView>,
    cancelled_reservations: Vec<ReservationView>,
    admins: Vec<UserView>,
    username: &'a str,
}

pub async fn admin_page(auth_session: AuthenticatedSession) -> AppResult<Html<String>> {
    let user = auth_session.user;
    let db_pool = auth_session.session.backend.db.pool;
    let res = SortedReservations::new(get_reservations(&db_pool).await?);
    let admins = get_users(&db_pool).await?;
    let overlaps = find_overlaps(&res);
    let res_overlaps = distribute_overlaps(&res, overlaps);

    let views: Vec<ReservationView> = res
        .iter()
        .zip(res_overlaps)
        .map(|(r, o)| ReservationView::from_domain(r, &o))
        .collect();

    Ok(Html(
        AdminTemplate {
            reservations: views.iter().filter(|r| !r.approved && !r.cancelled).cloned().collect(),
            approved_reservations: views.iter().filter(|r| r.approved && !r.cancelled).cloned().collect(),
            cancelled_reservations: views.iter().filter(|r| r.cancelled).cloned().collect(),
            admins: admins
                .iter()
                .map(|u| UserView::from_domain(u.id, u.username.clone()))
                .collect(),
            username: user.username.as_str(),
        }
        .render()?,
    ))
}

pub async fn approve_reservation(
    auth_session: AuthenticatedSession,
    Path(id): Path<i32>,
) -> AppResult<Redirect> {
    crate::reservation::approve_reservation_db::approve_reservation(
        &auth_session.session.backend.db.pool,
        true,
        id,
    )
    .await?
    .ok_or(AppError::NotFound)?;

    info!("Approved reservation: {id}");
    Ok(Redirect::to("/admin"))
}
