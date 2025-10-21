use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Reservation {
    pub id: i32,
    pub reservation_name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

// Request
#[derive(Deserialize)]
pub struct CreateReservation {
    pub reservation_name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ReservationValidation {
    pub is_valid: bool,
    pub reason: String,
}