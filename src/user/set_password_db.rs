use sqlx::PgPool;
use tracing::debug;

use crate::{
    auth::types::HashedPassword,
    db::{DbError, UpdateOutcome},
};

pub async fn set_password(
    db_pool: &PgPool,
    user_id: i32,
    password: HashedPassword,
) -> Result<UpdateOutcome, DbError> {
    debug!(user_id, "Try to set user password");
    let res = sqlx::query!(
        "UPDATE users SET password = $1 WHERE id = $2",
        password.as_str(),
        user_id,
    )
    .execute(db_pool)
    .await?;

    Ok(UpdateOutcome::from(res))
}
