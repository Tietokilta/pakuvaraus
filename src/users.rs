use serde::{Deserialize, Serialize};
use axum_login::{AuthUser, AuthSession};
use password_auth::{verify_password, generate_hash};
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

#[derive(Deserialize)]
pub struct SetUserPasswordData {
    old_password: String,
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

pub fn password_hash(password: &String) -> String {
    return generate_hash(password)
}

pub fn check_user_credentials(user: &User, creds: &Credentials) -> bool {
    return verify_password(&creds.password, &user.password).is_ok();
    //return true;
}

pub async fn add_user(
    auth_session: &AuthSession<AppState>,
    user: &CreateUserData
) {
    info!("Try to create new user: {}", user.username);
    match &auth_session.user {
        Some(_) => {
            if user.username.len() != 0 && user.password == user.password_again {
                let res = sqlx::query_as!(
                    User,
                    "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id, username, password",
                    &user.username,
                    password_hash(&user.password)
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

pub async fn set_user_password(
    auth_session: &AuthSession<AppState>,
    data: &SetUserPasswordData
) {
    info!("Try to set user password");
    match &auth_session.user {
        Some(user) => {
            if data.password == data.password_again {
                let res = sqlx::query!(
                    "UPDATE users SET password = $1 WHERE id = $2",
                    data.password,
                    user.id,
                )
                .fetch_one(&auth_session.backend.db_pool)
                .await;

                match res {
                    Ok(_) => {
                        info!("Password set successfully for user: {}", user.username);                        
                    },
                    Err(err) => {
                        info!("Set password failed, database error: {}", err.to_string());
                    }
                }
            } else {
                info!("Set password data not valid")
            }
        },
        None => info!("Not authorized")
    }
}

pub async fn remove_user(
    auth_session: &AuthSession<AppState>,
    username: &String,
) {
    info!("Try to remove user");
    match &auth_session.user {
        Some(_) => {
            if username.len() != 0 {
                let res = sqlx::query!(
                    "DELETE FROM users WHERE username = $1",
                    username
                )
                .fetch_one(&auth_session.backend.db_pool)
                .await;

                match res {
                    Ok(_) => {
                        info!("User successfully deleted: {}", username);                        
                    },
                    Err(err) => {
                        info!("Remove user failed, database error: {}", err.to_string());
                    }
                }
            } else {
                info!("Remove username not valid")
            }
        },
        None => info!("Not authorized")
    }
}