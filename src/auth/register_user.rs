use sqlx::PgPool;

use super::types::PlainPassword;
use super::password::get_password_hash;
use crate::user::add_user_db::add_user;

use tracing::info;

pub async fn register_user(
    db_pool: &PgPool,
    username: &String,
    password: PlainPassword,
    password_again: PlainPassword,
) {
    if password.0 == password_again.0 {
        add_user(db_pool, username, get_password_hash(&password)).await;
    }
    else {
        info!("Register user failed, password does not match");
    }
}
