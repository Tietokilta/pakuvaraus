use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::{debug, error};
use utoipa::ToSchema;

use crate::auth::change_password::ChangePasswordError;
use crate::db::DbError;

/// Body of every error response that the app itself produces.
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Human-readable description of the error
    pub error: String,
}

pub type AppResult<T, E = AppError> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("invalid username or password")]
    InvalidCredentials,
    #[error("the requested resource was not found")]
    NotFound,
    #[error("{0}")]
    Conflict(&'static str),

    // Server errors
    #[error("an error occured with the database")]
    Sqlx(#[from] sqlx::Error),
    #[error("an error occured while sending email")]
    Email(#[from] crate::email::EmailError),
    #[error("an error occured rendering HTML")]
    Askama(#[from] askama::Error),
    #[error("an internal server error occured")]
    Anyhow(#[from] anyhow::Error),
}

impl From<DbError> for AppError {
    fn from(value: DbError) -> Self {
        match value {
            DbError::Constraint(s) => Self::Conflict(s),
            DbError::Sqlx(error) => Self::Sqlx(error),
        }
    }
}

impl From<ChangePasswordError> for AppError {
    fn from(value: ChangePasswordError) -> Self {
        match value {
            ChangePasswordError::WrongPassword => Self::InvalidCredentials,
            ChangePasswordError::DbError(e) => e.into(),

            // User not found when the user already has an auth session is a 5xx error
            other @ (ChangePasswordError::UserNotFound
            | ChangePasswordError::MalformedHash(_)
            | ChangePasswordError::TokioError(_)) => Self::Anyhow(other.into()),
        }
    }
}

impl AppError {
    const fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Sqlx(_) | Self::Anyhow(_) | Self::Askama(_) | Self::Email(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        if status.is_server_error() {
            error!(?self);
        } else {
            debug!(?self);
        }

        (
            status,
            axum::Json(ErrorResponse {
                error: self.to_string(),
            }),
        )
            .into_response()
    }
}
