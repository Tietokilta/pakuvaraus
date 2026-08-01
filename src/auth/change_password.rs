use sqlx::PgPool;

use super::types::PlainPassword;
use super::password::get_password_hash;
use crate::user::set_password_db::set_password;

use tracing::info;

pub async fn change_password(
    db_pool: &PgPool,
    user_id: i32,
    current_password: PlainPassword,
    old_password: PlainPassword,
    new_password: PlainPassword,
    new_password_again: PlainPassword,
) {
    if current_password.0 == old_password.0 && new_password.0 == new_password_again.0 {
        set_password(db_pool, user_id, get_password_hash(new_password)).await;
    }
    else {
        info!("Register user failed, password does not match");
    }
}