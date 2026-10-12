use serde::Deserialize;

use crate::{auth::types::HashedPassword, domain::Username};

#[derive(Clone, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: Username,
    pub password: HashedPassword,
}

#[derive(Deserialize)]
pub struct UserDataObject {
    pub id: i32,
    pub username: String,
}
