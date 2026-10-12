use crate::db::service::DbService;
use crate::email::service::EmailService;

#[derive(Clone)]
pub struct AppState {
    pub db: DbService,
    pub email: EmailService,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            db: DbService::create().await,
            email: EmailService::create(),
        }
    }
}
