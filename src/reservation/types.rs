use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::domain::email::Email;

#[derive(sqlx::FromRow)]
pub struct Reservation {
    pub id: i32,
    pub reservation_name: String,
    pub email: Email,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateReservation {
    pub reservation_name: String,
    pub email: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct ReservationItem {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

impl From<Reservation> for ReservationItem {
    fn from(r: Reservation) -> Self {
        Self {
            start_time: r.start_time,
            end_time: r.end_time,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReservationValidation {
    pub is_valid: bool,
    pub reason: String,
}

pub struct ValidationResult {
    pub valid: ReservationValidation,
    pub reservation: Option<Reservation>,
}