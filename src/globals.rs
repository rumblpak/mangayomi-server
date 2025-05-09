use lazy_static::lazy_static;
use std::env;

lazy_static! {
    /// The complete database connection string: e.g., mysql://user:pw@localhost:3306/database
    pub static ref DATABASE_URL: String = env::var("DATABASE_URL").unwrap_or("sqlite://app.db?mode=rwc".to_string());
    /// The host ip address this server will use
    pub static ref HOST: String = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    /// The port this server will use
    pub static ref PORT: String = env::var("PORT").unwrap_or("8080".to_string());
}
