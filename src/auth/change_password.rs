use sqlx::PgPool;

use super::types::{PlainPassword, PasswordHash};
use super::password::{get_password_hash, check_plain_password};
use crate::user::set_password_db::set_password;

use tracing::info;

pub async fn change_password(
    db_pool: &PgPool,
    user_id: i32,
    current_password: PasswordHash,
    old_password: PlainPassword,
    new_password: PlainPassword,
    new_password_again: PlainPassword,
) {
    if check_plain_password(old_password, current_password) {
        if new_password.0 == new_password_again.0 {
            set_password(db_pool, user_id, get_password_hash(&new_password)).await;
        } 
        else {
            info!("Change password failed, password does not match");
        }
    }
    else {
        info!("Change password failed, incorrect password\n");
    }
}