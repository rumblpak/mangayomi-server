use crate::db;
use crate::user::model::BasicUser;
use crate::user::service::{login_account, register_account};
use actix_http::HttpMessage;
use actix_identity::Identity;
use actix_web::{HttpRequest, Result, get, post, web};
use mongodb::Client;

/// register a new account with the given email and password
#[post("/register")]
async fn register(client: web::Data<Client>, user: web::Json<BasicUser>) -> Result<String> {
    let result = register_account(client, &user);
    match result.await {
        Some(_account) => Ok("Account registered!".to_owned()),
        None => Ok(format!("Account already exists {}!", user.email)),
    }
}

/// login into the session
#[post("/login")]
async fn login(request: HttpRequest, client: web::Data<Client>, user: web::Json<BasicUser>) -> Result<String> {
    let result = login_account(client, &user);
    match result.await {
        Some(account) => {
            Identity::login(&request.extensions(), account.id.to_string())?;
            Ok(format!("Welcome {}!", account.email))
        }
        None => Ok(format!("Account not found {}!", user.email)),
    }
}

/// logout from session
#[get("/logout")]
async fn logout(user: Identity) -> Result<String> {
    user.logout();
    Ok("Logged out!".to_owned())
}

/// logout from session
#[get("/")]
async fn home(user: Identity) -> Result<String> {
    Ok(format!("Welcome {}!", user.id()?))
}

/// logout from session
#[get("/unprotected")]
async fn unprotected() -> Result<String> {
    Ok("Test".to_owned())
}
