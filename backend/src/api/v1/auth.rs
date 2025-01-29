use std::future::Future;
use std::pin::Pin;

use crate::api::ApiResponse;
use crate::models::{FrontendUser, SurrealDBUser};
use crate::{db::SurrealDb, prelude::*};

use crate::jwt::TokenFactory;
use crate::FinanalizeError;
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpRequest, Responder,
};
use actix_web::{FromRequest, HttpMessage};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct AccessToken {
    access_token: String,
}

#[derive(Serialize, Deserialize)]
struct UserForm {
    email: String,
    password: String,
}

impl FromRequest for SurrealDBUser {
    type Error = FinanalizeError;

    type Future = Pin<Box<dyn Future<Output = Result<Self>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let id_opt = req.extensions().get::<String>().cloned();
        let db_opt = req.app_data::<Data<SurrealDb>>().cloned();

        Box::pin(async move {
            let id = id_opt.ok_or(FinanalizeError::Unauthorized(AuthError::InvalidToken))?;
            let db = db_opt.ok_or(FinanalizeError::InternalServerError)?;

            db.select(("user", id))
                .await?
                .ok_or(FinanalizeError::Unauthorized(AuthError::InvalidToken))
        })
    }
}

#[post("/refresh")]
pub async fn refresh(token_factory: Data<TokenFactory>, req: HttpRequest) -> impl Responder {
    let refresh_token = match req.cookie("refresh_token") {
        Some(cookie) => cookie.value().to_string(),
        None => return Err(FinanalizeError::Unauthorized(AuthError::InvalidToken)),
    };

    let subject = token_factory.subject(&refresh_token)?;
    let pair = token_factory.generate_token(subject)?;
    Ok(ApiResponse::new(AccessToken {
        access_token: pair.access().to_string(),
    })
    .with_cookie(format!(
        "refresh_token={}; HttpOnly; Path=/; Max-Age=3600; SameSite=Strict",
        pair.refresh()
    )))
}

#[post("/login")]
pub async fn login(
    db: Data<SurrealDb>,
    token_factory: Data<TokenFactory>,
    user: Json<UserForm>,
) -> impl Responder {
    // Validate email and password
    // Fetch user from DB
    let mut response = db
        .query("SELECT * FROM user WHERE email = $email")
        .bind(("email", user.email.clone()))
        .await?;

    let Some(existing) = response.take::<Option<SurrealDBUser>>(0)? else {
        return Err(FinanalizeError::Unauthorized(AuthError::InvalidCredentials));
    };

    // Verify password hash
    let parsed_hash = PasswordHash::new(&existing.password)?;
    Argon2::default().verify_password(user.password.as_bytes(), &parsed_hash)?;

    // Generate token pair and set cookie with refresh token, return access token
    let pair = token_factory.generate_token(existing.id.id.to_string())?;

    Ok(ApiResponse::new(AccessToken {
        access_token: pair.access().to_string(),
    })
    .with_cookie(format!(
        "refresh_token={}; HttpOnly; Path=/; Max-Age=3600; SameSite=Strict",
        pair.refresh()
    )))
}

#[post("/register")]
pub async fn register(
    db: Data<SurrealDb>,
    token_factory: Data<TokenFactory>,
    mut user: Json<UserForm>,
) -> Result<impl Responder> {
    let exists = db
        .query("SELECT * FROM user WHERE email = $email")
        .bind(("email", user.email.clone()))
        .await?
        .take::<Option<SurrealDBUser>>(0)?
        .is_some();

    if exists {
        return Err(FinanalizeError::Unauthorized(AuthError::EmailAlreadyExists));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(user.password.as_bytes(), &salt)?
        .to_string();
    user.password = hash;

    let user: SurrealDBUser = db
        .create("user")
        .content(user)
        .await?
        .ok_or(FinanalizeError::InternalServerError)?;

    let pair = token_factory.generate_token(user.id.id.to_string())?;

    Ok(ApiResponse::new(AccessToken {
        access_token: pair.access().to_string(),
    })
    .with_cookie(format!(
        "refresh_token={}; HttpOnly; Path=/; Max-Age=3600; SameSite=Strict",
        pair.refresh()
    )))
}

#[post("/logout")]
pub async fn logout() -> Result<impl Responder> {
    Ok(ApiResponse::new(
        "Logged out successfully"
    ).with_cookie(
        "refresh_token=Deleted; HttpOnly; Path=/; Max-Age=0; SameSite=Strict; Expires=Thu, 01 Jan 1970 00:00:00 GMT".to_string(),
    ))
}

#[get("/me")]
pub async fn me(user: SurrealDBUser) -> impl Responder {
    let user: FrontendUser = user.into();
    ApiResponse::new(user)
}
