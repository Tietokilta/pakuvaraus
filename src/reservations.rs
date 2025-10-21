use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Reservation {
    pub id: i32,
    pub reservation_name: String,
}

// Request
#[derive(Deserialize)]
pub struct CreateReservation {
    pub reservation_name: String,
}