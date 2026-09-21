use super::ParseError;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "RawTimeRange")]
pub struct TimeRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

#[derive(Deserialize)]
struct RawTimeRange {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

impl TimeRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self, ParseError> {
        if start >= end {
            return Err(ParseError::TimeRange);
        }
        Ok(Self { start, end })
    }

    pub const fn start(&self) -> DateTime<Utc> {
        self.start
    }

    pub const fn end(&self) -> DateTime<Utc> {
        self.end
    }
}

impl TryFrom<RawTimeRange> for TimeRange {
    type Error = ParseError;

    fn try_from(raw: RawTimeRange) -> Result<Self, Self::Error> {
        Self::new(raw.start_time, raw.end_time)
    }
}
