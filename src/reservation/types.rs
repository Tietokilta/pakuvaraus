use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use utoipa::ToSchema;

use crate::{
    domain::{Address, Email, FullName, Hetu, Phone, RawTimeRange, TimeRange},
    email::email_templates::ReservationEmail,
};
use std::ops::Deref;

//Validated

#[derive(sqlx::FromRow)]
#[allow(clippy::struct_field_names, unused)]
pub struct Reservation {
    pub id: i32,
    pub full_name: FullName,
    pub email: Email,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub internal: bool,
    pub approved: bool,
    pub cancelled: bool,
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

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateReservation {
    #[schema(value_type = String, min_length = 1, max_length = 512)]
    pub full_name: FullName,
    #[schema(value_type = String, min_length = 1, max_length = 128)]
    pub address: Address,
    #[schema(value_type = String, format = Email)]
    pub email: Email,
    /// Phone number; a number without a country code is read as Finnish
    #[schema(value_type = String)]
    pub phone_number: Phone,
    /// Finnish personal identity code (henkilötunnus)
    #[schema(value_type = String)]
    pub hetu: Hetu,
    // `start_time` and `end_time` in the JSON
    #[serde(flatten)]
    #[schema(value_type = RawTimeRange, inline)]
    pub time: TimeRange,
    /// Reserved for guild use rather than a member's personal use
    pub internal: bool,
    pub used_outside_finland: bool,
}

impl<'a> From<&'a CreateReservation> for ReservationEmail<'a> {
    fn from(value: &'a CreateReservation) -> Self {
        Self {
            email: &value.email,
            name: &value.full_name,
            phone_number: &value.phone_number,
            timerange: &value.time,
            used_outside_finland: value.used_outside_finland,
        }
    }
}

#[derive(Serialize, ToSchema)]
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

//Overlaps

#[derive(Copy, Clone)]
pub struct Overlap {
    pub other_id: i32,
    pub duration: chrono::Duration,
}

//Edit key

pub struct EditKey(String);

#[derive(Clone, PartialEq, Eq)]
pub struct EditKeyHash([u8; 32]);

impl EditKey {
    pub fn new() -> Self {
        let mut bytes = [0u8; 32];
        rand::fill(&mut bytes);
        Self(URL_SAFE_NO_PAD.encode(bytes))
    }

    pub fn parse(s: &str) -> Option<Self> {
        let bytes = URL_SAFE_NO_PAD.decode(s).ok()?;
        (bytes.len() == 32).then(|| Self(s.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn hash(&self) -> EditKeyHash {
        EditKeyHash(Sha256::digest(self.0.as_bytes()).into())
    }
}

impl EditKeyHash {
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}
