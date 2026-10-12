use std::env;

pub struct DbConfig {
    pub url: String,
}

impl DbConfig {
    pub fn from_env() -> Self {
        Self {
            url: env::var("DATABASE_URL").expect("DATABASE_URL env not set"),
        }
    }
}
