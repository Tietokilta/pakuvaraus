use sqlx::postgres::PgPool;

use crate::db::{DbError, ResultExt};
use crate::reservation::types::{EditKey, EditKeyHash};

pub async fn set_cancel_by_key(db_pool: &PgPool, key: &EditKeyHash, cancel: bool) -> Result<bool, DbError> {
    let res = sqlx::query!(
        r#"
        UPDATE reservations SET cancelled = $2 WHERE edit_key = $1
        "#,
        key.as_bytes().as_slice(), cancel,
    )
    .execute(db_pool)
    .await?;
    
    Ok(res.rows_affected() == 1)
}