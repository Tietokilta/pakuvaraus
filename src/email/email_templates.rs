use askama::Template;

use crate::{
    domain::{Email, FullName, Phone, TimeRange},
    email::{EmailError, attachment::Attachment},
};

use super::service::EmailService;

#[derive(Template)]
#[template(path = "email/contract_email.html")]
pub struct ContractEmail<'a> {
    pub reservation: &'a ReservationEmail<'a>,
    pub contract_bytes: Vec<u8>,
}

pub async fn send_contract_email(
    service: &EmailService,
    data: ContractEmail<'_>,
) -> Result<(), EmailError> {
    let html = data.render()?;
    let subject = format!("New van reservation request - {}", data.reservation.name);
    let attachment = Attachment {
        filename: format!("Van reservation contract - {}", data.reservation.name),
        content_type: "application/pdf",
        data: data.contract_bytes,
    };
    service
        .sender
        .send(
            service.config.approver_email.to_string(),
            subject,
            html,
            Box::new([attachment]),
        )
        .await
}

#[derive(Template)]
#[template(path = "email/reservation.html")]
pub struct ReservationEmail<'a> {
    pub email: &'a Email,
    pub name: &'a FullName,
    pub phone_number: &'a Phone,
    pub timerange: &'a TimeRange,
    pub used_outside_finland: bool,
}

#[derive(Template)]
#[template(path = "email/confirmation_email.html")]
pub struct ReservationConfirmationEmail<'a> {
    pub reservation: &'a ReservationEmail<'a>,
    pub cancel_link: String,
}

pub async fn send_reservation_email(
    service: &EmailService,
    data: ReservationConfirmationEmail<'_>,
) -> Result<(), EmailError> {
    let html = data.render()?;

    service
        .sender
        .send(
            data.reservation.email.to_string(),
            "Tietokilta van reservation request received".to_string(),
            html,
            Box::new([]),
        )
        .await
}
