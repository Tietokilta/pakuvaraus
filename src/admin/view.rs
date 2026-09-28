use chrono::{DateTime, Duration, Utc};

use crate::reservation::types::{Reservation, Overlap};

//Helper

fn format_time(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M").to_string()
}

fn format_duration(dur: chrono::Duration) -> String {
    let total_minutes = dur.num_minutes();
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;

    format!("{hours}h {minutes}m")
}

//View

#[derive(Clone)]
pub struct UserView {
    pub id: i32,
    pub username: String,
}

#[derive(Clone)]
pub struct OverlapView {
    pub id: i32,
    pub duration: String,
}

#[derive(Clone)]
pub struct ReservationView {
    pub id: i32,
    pub start_time: String,
    pub end_time: String,
    pub duration: String,
    pub email: String,
    pub internal: bool,
    pub approved: bool,
    pub overlaps: Vec<OverlapView>,
}

//Conversion

impl UserView {
    pub const fn from_domain(id: i32, username: String) -> Self {
        Self { id, username }
    }
}

impl OverlapView {
    pub fn from_domain(o: &Overlap) -> Self {
        Self { 
            id: o.other_id,
            duration: format_duration(o.duration),
        }
    }
}

impl ReservationView {
    pub fn from_domain(r: &Reservation, overlaps: &Vec<Overlap>) -> Self {
        let duration = r.end_time - r.start_time;

        Self {
            id: r.id,
            start_time: format_time(r.start_time),
            end_time: format_time(r.end_time),
            duration: format_duration(duration),
            email: r.email.as_str().to_string(),
            internal: r.internal,
            approved: r.approved,
            overlaps: overlaps.iter().map(|o| OverlapView::from_domain(o)).collect(),
        }
    }
}