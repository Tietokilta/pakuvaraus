mod phone;
mod hetu;
mod email;
mod non_empty_text;
mod time_range;
mod simple;

pub use phone::*;
pub use hetu::*;
pub use email::*;
pub use non_empty_text::*;
pub use time_range::*;
pub use simple::*;

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
