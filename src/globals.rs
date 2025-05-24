use lazy_static::lazy_static;
use std::env;

lazy_static! {
    /// The complete database connection string: e.g., mysql://user:pw@localhost:3306/database
    pub static ref DATABASE_URL: String = env::var("DATABASE_URL").unwrap_or("sqlite://app.db?mode=rwc".to_string());
    /// The host ip address this server will use
    pub static ref HOST: String = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    /// The port this server will use
    pub static ref PORT: String = env::var("PORT").unwrap_or("8080".to_string());
    /// How many days the user stays logged in without refreshing
    pub static ref SESSION_TTL: i64 = env::var("SESSION_TTL_DAYS").unwrap_or("30".to_string()).parse().expect("Failed to read SESSION_TTL_DAYS");
    /// Whether the server should use a cookie-based or redis-based backend
    pub static ref USE_REDIS: bool = env::var("USE_REDIS").unwrap_or("false".to_string()).parse().expect("Failed to read USE_REDIS");
    /// The redis connection string this server will use
    pub static ref REDIS_URL: String = env::var("REDIS_URL").unwrap_or("redis://127.0.0.1:6379".to_string());
}
