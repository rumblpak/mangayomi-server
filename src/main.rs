use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use sea_orm::{ConnectOptions, Database};
use std::{env, time::Duration};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT")
        .expect("PORT is not set in .env file")
        .parse::<u16>()
        .unwrap();

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

    let conn = Database::connect(opt)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .service(hello)
    })
    .bind((host, port))?
    .run()
    .await
}
