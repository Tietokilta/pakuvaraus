use serde::Deserialize;

#[derive(Clone, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserDataObject {
    pub id: i32,
    pub username: String,
}

