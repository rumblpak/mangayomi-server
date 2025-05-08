use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;
use url::Url;

lazy_static! {
    pub static ref DATABASE_URL: String = &env::var("DATABASE_URL").unwrap_or("sqlite://app.db?mode=rwc".to_string()).expect("DATABASE_URL is not set in .env file");
    pub static ref HOST: String = &env::var("HOST").unwrap_or("0.0.0.0".to_string()).expect("HOST is not set in .env file");
    pub static ref PORT: u16 = &env::var("PORT").unwrap_or(8080).expect("PORT is not set in .env file");
}
