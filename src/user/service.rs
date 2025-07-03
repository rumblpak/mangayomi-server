use crate::user::model::User;
use actix_web::web;
use argon2::Argon2;
use mongodb::Client;
use mongodb::bson::doc;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use std::time::{SystemTime, UNIX_EPOCH};

/// inserts a new account if it does not exist yet
pub async fn register_account(
    db: web::Data<Client>,
    user: &web::Json<crate::user::model::BasicUser>,
) -> Option<User> {
    let usr = find_account(&user.email, &db).await;
    if usr.is_none() {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(user.password.as_bytes(), &salt)
            .expect("Failed to hash password!");
        let collection = db.database("mangayomi").collection("users");
        let timestamp = i64::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        )
        .unwrap();
        let account = User {
            id: None,
            email: user.email.to_owned(),
            password: password_hash.to_string(),
            salt: salt.to_string(),
            role: "BASIC".to_string(),
            created_at: timestamp,
            updated_at: timestamp,
        };
        return match collection.insert_one(account).await {
            Ok(_result) => find_account(&user.email, &db).await,
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
    db: web::Data<Client>,
    user: &web::Json<crate::user::model::BasicUser>,
) -> Option<User> {
    let result = find_account(&user.email, &db).await;
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
async fn find_account(email: &String, db: &Client) -> Option<User> {
    let collection = db.database("mangayomi").collection("users");
    match collection.find_one(doc! { "email": email }).await {
        Ok(Some(user)) => Some(user),
        Ok(None) => None,
        Err(err) => {
            log::error!("{}", err);
            None
        }
    }
}
