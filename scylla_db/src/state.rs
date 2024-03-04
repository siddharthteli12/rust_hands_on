use lazy_static::lazy_static;
use std::env;
lazy_static! {
    pub static ref APP_STATE: State = State::build();
}

pub struct State {
    pub host: String,
    pub port: String,
    pub db_host: String,
    pub db_port: String,
}

impl State {
    pub fn build() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| String::from("0.0.0.0")),
            port: env::var("URL").unwrap_or_else(|_| String::from("8080")),
            db_host: env::var("DB_HOST").unwrap_or_else(|_| String::from("0.0.0.0")),
            db_port: env::var("DB_PORT").unwrap_or_else(|_| String::from("9042")),
        }
    }
}
