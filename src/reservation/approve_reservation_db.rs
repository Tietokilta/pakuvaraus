use sqlx::postgres::PgPool;

pub async fn approve_reservation(
    db_pool: &PgPool,
    state: bool,
    id: i32
) {
    let _ = sqlx::query!(
        "UPDATE reservations SET approved = $1 WHERE id = $2",
        state,
        id
    )
    .execute(db_pool)
    .await;
}