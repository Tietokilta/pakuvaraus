use sqlx::postgres::PgQueryResult;

#[must_use = "check if the DB query updated something"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateOutcome {
    Updated,
    NotFound,
}

impl UpdateOutcome {
    pub fn ok_or<E>(self, err: E) -> Result<(), E> {
        match self {
            Self::Updated => Ok(()),
            Self::NotFound => Err(err),
        }
    }
}

impl From<PgQueryResult> for UpdateOutcome {
    fn from(value: PgQueryResult) -> Self {
        if value.rows_affected() >= 1 {
            Self::Updated
        } else {
            Self::NotFound
        }
    }
}
