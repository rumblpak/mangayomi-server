use crate::db;
use crate::sync::category::model::CategoryList;
use crate::sync::category::service::sync_category_list;
use actix_identity::Identity;
use actix_web::{HttpResponse, Responder, post, web};

#[post("/category")]
async fn sync_category(user: Identity, category_list: web::Json<CategoryList>) -> impl Responder {
    let result = sync_category_list(
        (&user.id().unwrap()).parse().unwrap(),
        &category_list.categories,
        db::CONN.get().unwrap(),
    );
    HttpResponse::Ok().json(CategoryList {
        categories: result.await,
    })
}
