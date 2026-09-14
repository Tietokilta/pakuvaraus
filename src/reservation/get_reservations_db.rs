use sqlx::postgres::PgPool;

use crate::reservation::types::Reservation;

pub async fn get_reservations(db_pool: &PgPool) -> Vec<Reservation> {
    sqlx::query_as!(Reservation, "SELECT id, reservation_name, email, start_time, end_time, internal, approved FROM reservations")
        .fetch_all(db_pool)
        .await
        .expect("Failed to fetch reservations")
}
