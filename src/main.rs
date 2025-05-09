use actix_web::{App, HttpServer, Responder, web};
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
        std::env::set_var("RUST_LOG", "info");
    }

    let db_url = globals::DATABASE_URL.as_str();
    let host = globals::HOST.as_str();
    let port = globals::PORT.as_str();

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
                .sqlx_logging_level(log::LevelFilter::Info)
                .set_schema_search_path("mangayomi");

            Database::connect(opt).await.unwrap()
        })
        .await;

    let conn = db::CONN.get().unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .service(user::controller::register)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
