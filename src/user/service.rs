use actix_web::web;
use argon2::Argon2;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

/// inserts a new account if it does not exist yet
pub async fn register_account(
    user: &web::Json<crate::user::model::BasicUser>,
    db: &DatabaseConnection,
) -> Option<crate::entity::accounts::Model> {
    let is_registered: bool = find_account(&user.email, db).await.is_some();
    if !is_registered {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(user.password.as_bytes(), &salt)
            .expect("Failed to hash password!");
        let account = crate::entity::accounts::ActiveModel {
            email: Set(user.email.to_owned()),
            password: Set(password_hash.to_string()),
            salt: Set(salt.to_string()),
            ..Default::default()
        };
        return match account.insert(db).await {
            Ok(model) => Some(model),
            Err(err) => {
                log::error!("{}", err);
                None
            }
        };
    }
    None
}

// returns account if the email and password matches
pub async fn login_account(
    user: &web::Json<crate::user::model::BasicUser>,
    db: &DatabaseConnection,
) -> Option<crate::entity::accounts::Model> {
    let result = find_account(&user.email, db).await;
    if result.is_some() {
        let account = result.unwrap();
        let hash = PasswordHash::new(&account.password).expect("Failed to hash password!");
        if Argon2::default()
            .verify_password(user.password.as_bytes(), &hash)
            .is_ok()
        {
            return Some(account);
        }
        return None;
    }
    None
}

/// returns an account with the matching email
async fn find_account(
    email: &String,
    db: &DatabaseConnection,
) -> Option<crate::entity::accounts::Model> {
    crate::entity::accounts::Entity::find()
        .filter(crate::entity::accounts::Column::Email.eq(email))
        .one(db)
        .await
        .map_or(None, |account| account)
}
