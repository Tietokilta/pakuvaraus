use sqlx::postgres::PgPool;

use crate::reservation::types::Reservation;

pub async fn get_reservations(db_pool: &PgPool) -> Vec<Reservation> {
    sqlx::query_as!(Reservation, r#"SELECT id, full_name AS "full_name: _", email AS "email: _", start_time, end_time, internal, approved FROM reservations"#)
        .fetch_all(db_pool)
        .await
        .expect("Failed to fetch reservations")
}
