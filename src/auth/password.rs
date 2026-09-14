use password_auth::{verify_password, generate_hash};

use super::types::{PasswordHash, PlainPassword};

pub fn get_password_hash(password: &PlainPassword) -> PasswordHash {
    PasswordHash(generate_hash(&password.0))
}

pub fn check_plain_password(password: PlainPassword, hash: &PasswordHash) -> bool {
    verify_password(password.0, hash.to_string()).is_ok()
}
