use askama::Template;

use crate::email::EmailError;

use super::service::EmailService;

#[derive(Template)]
#[template(path = "email/reservation.html")]
struct ReservationEmailTemplate<'a> {
    pub name: &'a str,
}

pub async fn send_reservation_email(service: &EmailService, to: &str) -> Result<(), EmailError> {
    let html = ReservationEmailTemplate { name: to }.render()?;

    service.sender.send(to, "Reservation received", &html).await
}

pub async fn send_approval_email(service: &EmailService, to: &str) -> Result<(), EmailError> {
    let html = ReservationEmailTemplate { name: to }.render()?;

    service.sender.send(to, "Reservation approved", &html).await
}
