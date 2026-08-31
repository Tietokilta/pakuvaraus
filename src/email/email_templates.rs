use askama::Template;

use super::service::EmailService;

#[derive(Template)]
#[template(path = "email/reservation.html")]
struct ReservationEmailTemplate<'a> {
    pub name: &'a str,
}

pub async fn send_reservation_email(service: &EmailService, to: &str) {
    let html = ReservationEmailTemplate {
        name: to,
    }.render().unwrap();

    service.sender.send(to, "Reservation received", &html).await;
}