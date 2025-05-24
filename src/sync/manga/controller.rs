use actix_web::{post, web, HttpResponse, Responder};
use crate::db;
use crate::sync::manga::model::MangaList;
use crate::user::service::register_account;

#[post("/manga")]
async fn sync_manga(user: web::Json<MangaList>) -> impl Responder {
    /*let result = register_account(&user, db::CONN.get().unwrap());
    match result.await {
        Some(_account) => Ok("Account registered!".to_owned()),
        None => Ok(format!("Account already exists {}!", "")),
    }*/
    HttpResponse::Ok().json({})
}
