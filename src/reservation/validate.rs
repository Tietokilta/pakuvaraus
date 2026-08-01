use super::types::{CreateReservation, ReservationValidation};

pub async fn validate_reservation(
    reservation: &CreateReservation
) -> ReservationValidation {
    if reservation.end_time < reservation.start_time { 
        return ReservationValidation {is_valid: false, reason: String::from("Start time must be before end time")}; 
    }

    ReservationValidation {is_valid: true, reason: String::from("Valid")}
}