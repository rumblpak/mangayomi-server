use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use sea_orm::{ConnectOptions, Database};
use std::time::Duration;

mod db;
mod entity;
mod globals;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    let db_url = globals::DATABASE_URL.as_str();
    let host = globals::HOST.as_str();
    let port = globals::PORT.as_str();

    log::info!("Connecting to {}:{}...", db_url, host);

    db::CONN
        .get_or_init(|| async {
            let mut opt = ConnectOptions::new(db_url);
            opt.max_connections(100)
                .min_connections(1)
                .connect_timeout(Duration::from_secs(8))
                .acquire_timeout(Duration::from_secs(8))
                .idle_timeout(Duration::from_secs(8))
                .max_lifetime(Duration::from_secs(8))
                .sqlx_logging(true)
                .sqlx_logging_level(log::LevelFilter::Debug);

            Database::connect(opt).await.unwrap()
        })
        .await;

    let conn = db::CONN.get().unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new(conn.clone()))
            .service(user::controller::register)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
