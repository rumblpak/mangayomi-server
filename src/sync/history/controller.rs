use crate::db;
use crate::sync::manga::model::MangaList;
use crate::sync::manga::service::sync_manga_list;
use actix_identity::Identity;
use actix_web::{HttpResponse, Responder, post, web};

#[post("/manga")]
async fn sync_manga(user: Identity, manga_list: web::Json<MangaList>) -> impl Responder {
    let result = sync_manga_list(
        (&user.id().unwrap()).parse().unwrap(),
        &manga_list.manga,
        db::CONN.get().unwrap(),
    );
    HttpResponse::Ok().json(MangaList {
        manga: result.await,
    })
}
