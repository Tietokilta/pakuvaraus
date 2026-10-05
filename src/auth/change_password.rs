use sqlx::PgPool;

use super::types::{NewPassword, PlainPassword, HashedPassword};
use super::password::{get_password_hash, check_plain_password};
use crate::user::set_password_db::set_password;

use tracing::info;

pub async fn change_password(
    db_pool: &PgPool,
    user_id: i32,
    current_password: HashedPassword,
    old_password: PlainPassword,
    new_password: &NewPassword,
) {
    if check_plain_password(old_password, &current_password) {
        set_password(db_pool, user_id, get_password_hash(new_password.as_plain())).await;
    }
    else {
        info!("Change password failed, incorrect password\n");
    }
}
