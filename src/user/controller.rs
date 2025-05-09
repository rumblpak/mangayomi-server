use actix_web::{post, web, Result};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;
use crate::entity::accounts::{self, Entity as Account};

#[derive(Deserialize)]
struct BasicUser {
    username: String,
    password: String,
}

/// deserialize `user` from request's body
#[post("/register")]
async fn register(user: web::Json<BasicUser>) -> Result<String> {
    Ok(format!("Welcome {}!", user.username))
}

/// WIP
async fn find_user(email: String, db: DatabaseConnection) {
    let account: Option<accounts::Model> = Account::find().one(&db).await.unwrap();
}
