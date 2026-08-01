use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserData {
    pub username: String,
    pub password: String,
    pub password_again: String,
}

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}