use actix_identity::IdentityMiddleware;
use actix_session::SessionMiddleware;
use actix_session::config::{CookieContentSecurity, PersistentSession, TtlExtensionPolicy};
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::{Key, SameSite};
use actix_web::middleware::{Logger, NormalizePath};
use actix_web::{App, HttpResponse, HttpServer, cookie::time::Duration as CookieDuration, web};
use sea_orm::{ConnectOptions, Database};
use std::time::Duration;

mod db;
mod entity;
mod globals;
mod user;
mod sync;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let db_url = globals::DATABASE_URL.as_str();
    let host = globals::HOST.as_str();
    let port = globals::PORT.as_str();
    let session_ttl = &globals::SESSION_TTL;
    // let redis_url = globals::REDIS_URL.as_str();

    let secret_key = Key::generate();

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
                .sqlx_logging_level(log::LevelFilter::Info);

            Database::connect(opt).await.unwrap()
        })
        .await;

    let conn = db::CONN.get().unwrap();

    /*
    if *globals::USE_REDIS {
                let redis_store = RedisSessionStore::new(redis_url)
                    .await
                    .unwrap();
                SessionMiddleware::new(
                    redis_store.clone(),
                    secret_key.clone(),
                )
            } else {
                SessionMiddleware::new(
                    CookieSessionStore::default(),
                    secret_key.clone(),
                )
            }
     */

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(CookieDuration::days(**session_ttl))
                            .session_ttl_extension_policy(TtlExtensionPolicy::OnEveryRequest),
                    )
                    .cookie_secure(true)
                    .cookie_same_site(SameSite::Strict)
                    .cookie_content_security(CookieContentSecurity::Private)
                    .cookie_http_only(true)
                    .build(),
            )
            .app_data(web::Data::new(conn.clone()))
            .service(user::controller::register)
            .service(user::controller::login)
            .service(user::controller::logout)
            .service(user::controller::home)
            .service(user::controller::unprotected)
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
