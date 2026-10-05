use crate::db::service::DbService;
use crate::email::service::EmailService;

#[derive(Clone)]
pub struct AppState {
    pub db: DbService,
    pub email: EmailService,
}

pub async fn new() -> AppState {
    AppState {
        db: DbService::create().await,
        email: EmailService::create(),
    }
}
