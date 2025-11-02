use serde::{Deserialize, Serialize};
use axum_login::{AuthUser, AuthSession};
use password_auth::verify_password;
use tracing::info;

use crate::state::AppState;

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

#[derive(Deserialize)]
pub struct CreateUserData {
    pub username: String,
    password: String,
    password_again: String,
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

pub async fn add_user(
    auth_session: &AuthSession<AppState>,
    user: &CreateUserData
) {
    info!("Try to create new user: {}", user.username);
    match &auth_session.user {
        Some(_) => {
            if(user.username.len() != 0 && user.password == user.password_again) {
                let res = sqlx::query_as!(
                    User,
                    "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id, username, password",
                    user.username,
                    user.password
                )
                .fetch_one(&auth_session.backend.db_pool)
                .await;

                match res {
                    Ok(_) => (),
                    Err(err) => {
                        info!("Create user failed, database error: {}", err.to_string());
                    }
                }
            } else {
                info!("Create user data not valid")
            }
        },
        None => info!("Not authorized")
    }
}
