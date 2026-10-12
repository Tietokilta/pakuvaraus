mod email;
mod hetu;
mod non_empty_text;
mod phone;
mod simple;
mod time_range;

pub use email::*;
pub use hetu::*;
pub use non_empty_text::*;
pub use phone::*;
pub use simple::*;
pub use time_range::*;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("invalid hetu")]
    Hetu,
    #[error("invalid phone number")]
    Phone,
    #[error("invalid email")]
    Email,
    #[error("must be between 1 and {0} characters")]
    TextLength(usize),
    #[error("end_time must be after start_time")]
    TimeRange,
    #[error("password must not be empty")]
    EmptyPassword,
    #[error("passwords do not match")]
    PasswordMismatch,
}
