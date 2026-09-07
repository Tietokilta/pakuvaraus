use chrono::{DateTime, Utc};

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

pub struct UserView {
    pub id: i32,
    pub username: String,
}

pub struct ReservationView {
    pub id: i32,
    pub start_time: String,
    pub end_time: String,
    pub duration: String,
    pub email: String,
}

//Conversion

impl UserView {
    pub const fn from_domain(id: i32, username: String) -> Self {
        Self { id, username }
    }
}

impl ReservationView {
    pub fn from_domain(
        id: i32,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        email: &str,
    ) -> Self {
        let duration = end - start;

        Self {
            id,
            start_time: format_time(start),
            end_time: format_time(end),
            duration: format_duration(duration),
            email: email.to_string(),
        }
    }
}
