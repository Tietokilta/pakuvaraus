use super::admin_page::{admin_page, approve_reservation};
use super::change_password::{change_password, get_change_password_page};
use super::new_admin::{get_new_admin_page, new_admin};
use super::remove_admin::remove_admin;
use super::remove_reservation::{remove_reservation, cancel_reservation, restore_reservation};
use crate::state::app_state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admin", get(admin_page))
        .route("/admin/{user_id}/remove", get(remove_admin))
        .route("/admin/new", get(get_new_admin_page))
        .route("/admin/new", post(new_admin))
        .route("/admin/password", get(get_change_password_page))
        .route("/admin/password", post(change_password))
        .route("/admin/reservations/{id}/remove", get(remove_reservation))
        .route("/admin/reservations/{id}/cancel", get(cancel_reservation))
        .route("/admin/reservations/{id}/restore", get(restore_reservation))
        .route("/admin/reservations/{id}/approve", get(approve_reservation))
}
