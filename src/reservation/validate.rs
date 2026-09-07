use super::types::{CreateReservation, ReservationValidation};
use crate::domain::email::Email;

pub async fn validate_reservation(
    reservation: &CreateReservation
) -> ReservationValidation {
    if reservation.end_time < reservation.start_time {
        return ReservationValidation {is_valid: false, reason: String::from("Start time must be before end time")};
    }

    let email = Email::parse(&reservation.email);
    if email.is_err() {
        return ReservationValidation {is_valid: false, reason: String::from("Email is not valid")};
    }

    ReservationValidation {is_valid: true, reason: String::from("Valid")}
}
