use sqlx::postgres::PgPool;

use crate::db::DbError;
use crate::reservation::types::EditKeyHash;

pub async fn set_cancel_by_key(
    db_pool: &PgPool,
    key: &EditKeyHash,
    cancel: bool,
) -> Result<bool, DbError> {
    let res = sqlx::query!(
        r#"
        UPDATE reservations SET cancelled = $2 WHERE edit_key = $1
        "#,
        key.as_bytes().as_slice(),
        cancel,
    )
    .execute(db_pool)
    .await?;

    Ok(res.rows_affected() == 1)
}

pub async fn set_cancel_by_id(
    db_pool: &PgPool,
    id: i32,
    cancel: bool,
) -> Result<bool, DbError> {
    let res = sqlx::query!(
        r#"
        UPDATE reservations SET cancelled = $2 WHERE id = $1
        "#,
        id,
        cancel,
    )
    .execute(db_pool)
    .await?;

    Ok(res.rows_affected() == 1)
}
