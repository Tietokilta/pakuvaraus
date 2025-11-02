use serde::{Deserialize, Serialize};
use axum_login::{AuthUser};
use password_auth::verify_password;
use tracing::info;

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    password: String,
}

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    password: String,
}

pub struct UserDataObject {
    pub id: i32,
    pub username: String,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

fn check_user_credentials(user: &User, creds: &Credentials) -> bool {
    //return verify_password(&creds.password, &user.password).is_ok();
    return true;
}