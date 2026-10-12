use axum_test::TestServer;
use pakuvaraus::{
    auth::{
        password::check_plain_password,
        types::{Credentials, NewPassword},
    },
    db::service::DbService,
    domain::Username,
    email::{config::EmailConfig, service::EmailService},
    state::app_state::AppState,
    user::get_users_db::get_user_by_name,
};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;

mod email;
pub mod fixtures;
pub use email::*;

pub const ADMIN_USER: &str = "admin";
pub const ADMIN_PASS: &str = "adminPass123";

pub fn new_password(s: &str) -> NewPassword {
    serde_json::from_value(json!({ "password": s, "password_again": s}))
        .expect("new_password to work")
}

pub fn create_test_server(pool: PgPool) -> (TestServer, Arc<TestSender>) {
    let sender = Arc::new(TestSender::new());
    let email_service = EmailService {
        sender: sender.clone(),
        config: EmailConfig {
            enabled: true,
            approver_email: email::APPROVER_EMAIL.parse().unwrap(),
        },
    };
    let state = AppState {
        db: DbService { pool },
        email: email_service,
    };
    let app = pakuvaraus::app(state);
    (TestServer::new(app).unwrap(), sender)
}

pub async fn create_authed_test_server(pool: PgPool) -> (TestServer, Arc<TestSender>) {
    create_admin(&pool).await;
    let (mut server, sender) = create_test_server(pool);
    server.save_cookies();

    let creds: Credentials = Credentials {
        username: Username::try_from(ADMIN_USER.to_string())
            .expect("ADMIN_USER to be a valid username"),
        password: ADMIN_PASS.to_string().into(),
    };

    let res = server.post("/login").form(&creds).await;
    res.assert_status_in_range(300..400);

    (server, sender)
}

pub async fn create_admin(pool: &PgPool) {
    let password = ADMIN_PASS.to_string().into();
    let hashed_password = pakuvaraus::auth::password::get_password_hash(password)
        .await
        .expect("failed to hash admin password for test");

    sqlx::query!(
        r#"
        INSERT INTO users (username, password)
        VALUES ($1, $2)
        "#,
        ADMIN_USER,
        hashed_password.as_str()
    )
    .execute(pool)
    .await
    .expect("failed to insert admin user into db");
}

pub async fn db_user_password_matches(pool: &PgPool, username: &str, password: String) -> bool {
    let user = get_user_by_name(pool, username)
        .await
        .expect("db fetch of user to work")
        .expect("db user to exist");

    check_plain_password(password.into(), user.password)
        .await
        .is_ok()
}

pub async fn db_user_exists(pool: &PgPool, username: &str) -> bool {
    get_user_by_name(pool, username)
        .await
        .expect("db fetch of user to work")
        .is_some()
}
