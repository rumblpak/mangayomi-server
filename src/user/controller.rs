use crate::db;
use crate::entity::accounts as account;
use actix_web::{Result, post, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize)]
struct BasicUser {
    email: String,
    password: String,
}

/// deserialize `user` from request's body
#[post("/register")]
async fn register(user: web::Json<BasicUser>) -> Result<String> {
    let result = register_account(&user, db::CONN.get().unwrap());
    match result.await {
        Some(account) => Ok(format!("Welcome {}!", account.email)),
        None => Ok(format!("User not found {}!", user.email)),
    }
}

/// inserts a new account if it does not exist yet
async fn register_account(
    user: &web::Json<BasicUser>,
    db: &DatabaseConnection,
) -> Option<account::Model> {
    let is_registered: bool = find_account(&user.email, db).await.is_some();
    if !is_registered {
        let account = account::ActiveModel {
            email: Set(user.email.to_owned()),
            password: Set(user.password.to_owned()),
            ..Default::default()
        };
        let account = account
            .insert(db)
            .await
            .map_or(None, |account| Option::from(account));
        return account;
    }
    None
}

/// returns an account with the matching email
async fn find_account(email: &String, db: &DatabaseConnection) -> Option<account::Model> {
    account::Entity::find()
        .filter(account::Column::Email.eq(email))
        .one(db)
        .await
        .map_or(None, |account| account)
}
