use password_auth::{verify_password, generate_hash};

use super::types::{HashedPassword, PlainPassword};

pub fn get_password_hash(password: &PlainPassword) -> HashedPassword {
    HashedPassword(generate_hash(&password.0))
}

pub fn check_plain_password(password: PlainPassword, hash: &HashedPassword) -> bool {
    verify_password(password.0, hash.as_str()).is_ok()
}
