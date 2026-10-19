use sqlx::postgres::PgPool;

use crate::{db::DbError, reservation::types::Reservation};

pub async fn get_reservations(db_pool: &PgPool) -> Result<Vec<Reservation>, DbError> {
    let res = sqlx::query_as!(
        Reservation,
        r#"
        SELECT id, full_name AS "full_name: _", email AS "email: _", start_time, end_time, internal, approved, cancelled
        FROM reservations
        "#
    )
        .fetch_all(db_pool)
        .await?;
    Ok(res)
}

pub async fn get_reservation_by_id(
    db_pool: &PgPool,
    id: i32,
) -> Result<Option<Reservation>, DbError> {
    let res = sqlx::query_as!(
        Reservation,
        r#"
        SELECT id, full_name AS "full_name: _", email AS "email: _", start_time, end_time, internal, approved, cancelled
        FROM reservations
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(db_pool)
    .await?;
    Ok(res)
}
