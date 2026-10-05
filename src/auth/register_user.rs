use sqlx::PgPool;

use super::password::get_password_hash;
use super::types::NewPassword;
use crate::{domain::Username, user::add_user_db::add_user};

pub async fn register_user(db_pool: &PgPool, username: &Username, password: &NewPassword) {
    add_user(db_pool, username, get_password_hash(password.as_plain())).await;
}
