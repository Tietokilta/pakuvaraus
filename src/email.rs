pub mod service;
pub mod email_templates;
pub mod attachment;

mod config;
mod test;

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Failed to send email")]
    Rejected {
        status: reqwest::StatusCode,
        body: String,
    },
    #[error(transparent)]
    Askama(#[from] askama::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}
