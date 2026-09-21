use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use std::ops::Deref;
use crate::domain::{Address, Email, FullName, Hetu, Phone, TimeRange};

//Validated

#[derive(sqlx::FromRow)]
#[allow(clippy::struct_field_names,unused)]
pub struct Reservation {
    pub id: i32,
    pub full_name: FullName,
    pub email: Email,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub internal: bool,
    pub approved: bool,
}

//Sorted validated reservations

pub struct SortedReservations(Box<[Reservation]>);

impl SortedReservations {
    pub fn new(mut v: Vec<Reservation>) -> Self {
        v.sort_by_key(|r| r.start_time);
        Self(v.into_boxed_slice())
    }
}

impl Deref for SortedReservations {
    type Target = [Reservation];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//Request

#[derive(Debug, Deserialize)]
pub struct CreateReservation {
    pub full_name: FullName,
    pub address: Address,
    pub email: Email,
    pub phone_number: Phone,
    pub hetu: Hetu,
    // `start_time` and `end_time` in the JSON
    #[serde(flatten)]
    pub time: TimeRange,
    pub internal: bool,
    pub used_outside_finland: bool,
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

//Validation

#[derive(Serialize, Deserialize)]
pub struct ReservationValidation {
    pub is_valid: bool,
    pub reason: String,
}

#[allow(unused)]
pub struct ValidationResult {
    pub valid: ReservationValidation,
    pub reservation: Option<Reservation>,
}

//Overlaps

#[derive(Clone)]
pub struct Overlap {
    pub other_id: i32,
    pub duration: chrono::Duration,
}
