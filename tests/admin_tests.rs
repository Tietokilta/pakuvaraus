use pakuvaraus::{
    auth::register_user::register_user,
    domain::Username,
    reservation::{
        add_reservation_db::add_reservation, get_reservations_db::get_reservation_by_id,
        types::EditKey,
    },
    user::get_users_db::get_user_by_id,
};
use reqwest::{Method, StatusCode};
use sqlx::PgPool;

use crate::common::{
    ADMIN_PASS, ADMIN_USER, db_user_exists, db_user_password_matches,
    fixtures::valid_create_reservation_json, new_password,
};

mod common;

#[sqlx::test]
async fn valid_change_password_works(pool: PgPool) {
    let (server, _) = common::create_authed_test_server(pool.clone()).await;
    let new_pass = "new_pass";
    server
        .post("/admin/password")
        .form(&[
            ("old_password", ADMIN_PASS),
            ("password", new_pass),
            ("password_again", new_pass),
        ])
        .await
        .assert_status_in_range(200..400);

    assert!(common::db_user_password_matches(&pool, ADMIN_USER, new_pass.to_string()).await);
}

#[sqlx::test]
async fn change_password_rejects_wrong_pass(pool: PgPool) {
    let (server, _) = common::create_authed_test_server(pool.clone()).await;
    let new_pass = "new_pass";
    let res = server
        .post("/admin/password")
        .form(&[
            ("old_password", "the_wrong_password"),
            ("password", new_pass),
            ("password_again", new_pass),
        ])
        .await;

    res.assert_status_in_range(200..400);
    assert!(common::db_user_password_matches(&pool, ADMIN_USER, ADMIN_PASS.to_string()).await);
}

#[sqlx::test]
async fn change_password_rejects_mismatched_new_pass(pool: PgPool) {
    let (server, _) = common::create_authed_test_server(pool.clone()).await;
    let res = server
        .post("/admin/password")
        .form(&[
            ("old_password", ADMIN_PASS),
            ("password", "password1"),
            ("password_again", "password2"),
        ])
        .await;

    res.assert_status_unprocessable_entity();
    assert!(res.text().contains("passwords do not match"));
    assert!(common::db_user_password_matches(&pool, ADMIN_USER, ADMIN_PASS.to_string()).await);
}

#[sqlx::test]
async fn valid_new_admin_works(pool: PgPool) {
    let (server, _) = common::create_authed_test_server(pool.clone()).await;
    let username = "newuser";
    let pass = "newpass";
    let res = server
        .post("/admin/new")
        .form(&[
            ("username", username),
            ("password", pass),
            ("password_again", pass),
        ])
        .await;

    res.assert_status_in_range(200..400);
    assert!(db_user_exists(&pool, username).await);
}

#[sqlx::test]
async fn new_admin_rejects_mismatched_new_pass(pool: PgPool) {
    let (server, _) = common::create_authed_test_server(pool.clone()).await;
    let username = "newuser";
    let res = server
        .post("/admin/new")
        .form(&[
            ("username", username),
            ("password", "pass1"),
            ("password_again", "pass2"),
        ])
        .await;

    res.assert_status_unprocessable_entity();
    assert!(res.text().contains("passwords do not match"));
    assert!(!db_user_exists(&pool, username).await);
}

#[sqlx::test]
async fn new_admin_rejects_existing_username(pool: PgPool) {
    let (server, _) = common::create_authed_test_server(pool.clone()).await;
    let pass = "newpass";
    let res = server
        .post("/admin/new")
        .form(&[
            ("username", ADMIN_USER),
            ("password", pass),
            ("password_again", pass),
        ])
        .await;

    res.assert_status_failure();
    assert!(res.text().contains("username already taken"));
    assert!(!db_user_password_matches(&pool, ADMIN_USER, pass.to_string()).await);
}

#[sqlx::test]
async fn remove_admin_works(pool: PgPool) {
    let (server, _) = common::create_authed_test_server(pool.clone()).await;
    let username = Username::try_from("newuser".to_string()).unwrap();
    let pass = new_password("newpass");
    let user = register_user(&pool, &username, pass)
        .await
        .expect("adding user to db to work");

    assert!(db_user_exists(&pool, username.as_str()).await);

    server
        .get(&format!("/admin/{}/remove", user.id))
        .await
        .assert_status_in_range(200..400);

    assert!(!db_user_exists(&pool, username.as_str()).await);
}

#[sqlx::test]
async fn remove_admin_rejects_nonexistent(pool: PgPool) {
    let (server, _) = common::create_authed_test_server(pool.clone()).await;

    let user_id = 0x1337;
    assert!(
        get_user_by_id(&pool, user_id)
            .await
            .expect("db call to work")
            .is_none()
    );

    server
        .get(&format!("/admin/{user_id}/remove"))
        .await
        .assert_status_not_found();
}

#[sqlx::test]
async fn remove_reservation_works(pool: PgPool) {
    let (server, _) = common::create_authed_test_server(pool.clone()).await;

    let reservation = add_reservation(
        &pool,
        &serde_json::from_value(valid_create_reservation_json()).unwrap(),
        EditKey::new().hash(),
    )
    .await
    .expect("adding reservation to db to work");

    server
        .get(&format!("/admin/reservations/{}/remove", reservation.id))
        .await
        .assert_status_in_range(200..400);

    assert!(
        get_reservation_by_id(&pool, reservation.id)
            .await
            .expect("fetching reservation from db to work")
            .is_none()
    );
}

#[sqlx::test]
async fn remove_reservation_rejects_nonexistent(pool: PgPool) {
    let (server, _) = common::create_authed_test_server(pool.clone()).await;
    let id = 0x1337;
    assert!(
        get_reservation_by_id(&pool, id)
            .await
            .expect("fetching reservation from db to work")
            .is_none()
    );

    server
        .get(&format!("/admin/reservations/{id}/remove"))
        .await
        .assert_status_not_found();
}

#[sqlx::test]
async fn unauthorized_admin_access_blocked(pool: PgPool) {
    let (server, _) = common::create_test_server(pool);
    let requests = [
        (Method::GET, "/admin"),
        (Method::GET, "/admin/1/remove"),
        (Method::GET, "/admin/new"),
        (Method::POST, "/admin/new"),
        (Method::GET, "/admin/password"),
        (Method::POST, "/admin/password"),
        (Method::GET, "/admin/reservations/1/remove"),
        (Method::GET, "/admin/reservations/1/approve"),
    ];

    for (method, path) in requests {
        let res = server.method(method, path).await;
        res.assert_status(StatusCode::TEMPORARY_REDIRECT);
        assert!(
            res.header("location")
                .to_str()
                .unwrap()
                .starts_with("/login")
        );
    }
}
