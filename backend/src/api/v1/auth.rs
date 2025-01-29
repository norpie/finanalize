use crate::models::SurrealDBUser;
use crate::{db::SurrealDb, prelude::*};

use crate::jwt::TokenFactory;
use crate::{models::ChangesetUser, FinanalizeError};
use actix_web::http::StatusCode;
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpRequest, HttpResponseBuilder, Responder,
};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::Serialize;

#[derive(Serialize)]
struct AccessToken {
    access_token: String,
}
#[post("/refresh")]
pub async fn refresh(token_factory: Data<TokenFactory>, req: HttpRequest) -> impl Responder {
    let refresh_token = match req.cookie("refresh_token") {
        Some(cookie) => cookie.value().to_string(),
        None => return Err(FinanalizeError::Unauthorized(AuthError::InvalidToken)),
    };

    let new_access_token = token_factory.generate_token_from_refresh(&refresh_token)?;
    let access_json = serde_json::to_string(&AccessToken {
        access_token: new_access_token.to_string(),
    })?;
    let api_response = HttpResponseBuilder::new(StatusCode::OK)
        .append_header((
            "Set-Cookie",
            format!(
                "refresh_token={}; HttpOnly; Path=/; Max-Age=3600; SameSite=Strict",
                refresh_token
            ),
        ))
        .json(access_json);

    Ok(api_response)
}

#[post("/login")]
pub async fn login(
    db: Data<SurrealDb>,
    token_factory: Data<TokenFactory>,
    user: Json<ChangesetUser>,
) -> impl Responder {
    // Validate email and password
    let email = match &user.email {
        Some(email) => email.clone(),
        None => return Err(FinanalizeError::Unauthorized(AuthError::MissingCredentials)),
    };

    let password = match &user.password {
        Some(password) => password.clone(),
        None => return Err(FinanalizeError::Unauthorized(AuthError::MissingCredentials)),
    };

    // Fetch user from DB
    let mut response = db
        .query("SELECT * FROM user WHERE email = $email")
        .bind(("email", email.clone()))
        .await?;

    let db_user: SurrealDBUser = match response.take::<Option<SurrealDBUser>>(0)? {
        Some(user) => user,
        None => return Err(FinanalizeError::Unauthorized(AuthError::InvalidCredentials)),
    };

    // Verify password hash
    let parsed_hash = PasswordHash::new(&db_user.password)?;
    let argon2 = Argon2::default();
    if argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(FinanalizeError::Unauthorized(AuthError::InvalidCredentials));
    }

    // Generate token pair and set cookie with refresh token, return access token
    let pair = token_factory.generate_token(db_user.id.id.to_string())?;
    let access_json = serde_json::to_string(&AccessToken {
        access_token: pair.access().to_string(),
    })?;

    let api_response = HttpResponseBuilder::new(StatusCode::OK)
        .append_header((
            "Set-Cookie",
            format!(
                "refresh_token={}; HttpOnly; Path=/; Max-Age=3600; SameSite=Strict",
                pair.refresh()
            ),
        ))
        .json(access_json);

    Ok(api_response)
}

#[post("/register")]
pub async fn register(
    db: Data<SurrealDb>,
    token_factory: Data<TokenFactory>,
    mut user: Json<ChangesetUser>,
) -> impl Responder {
    if user.email.is_none() || user.password.is_none() {
        return Err(FinanalizeError::Unauthorized(AuthError::MissingCredentials));
    }
    let mut response = db
        .query("SELECT * FROM user WHERE email = $email")
        .bind(("email", user.email.clone()))
        .await?;

    if response.take::<Option<SurrealDBUser>>(0)?.is_some() {
        return Err(FinanalizeError::Unauthorized(AuthError::EmailAlreadyExists));
    }

    let password = user.password.clone().unwrap();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    user.password = Some(hash);
    let user: SurrealDBUser = db.create("user").content(user).await?.unwrap();
    let pair = token_factory.generate_token(user.id.id.to_string())?;
    let access_json = serde_json::to_string(&AccessToken {
        access_token: pair.access().to_string(),
    })?;
    let api_response = HttpResponseBuilder::new(StatusCode::CREATED)
        .append_header((
            "Set-Cookie",
            format!(
                "refresh_token={}; HttpOnly; Path=/; Max-Age=3600; SameSite=Strict",
                pair.refresh()
            ),
        ))
        .json(access_json);

    Ok(api_response)
}

#[post("/logout")]
pub async fn logout() -> Result<impl Responder> {
    let api_response = HttpResponseBuilder::new(StatusCode::OK)
        .append_header((
            "Set-Cookie",
            "refresh_token=Deleted; HttpOnly; Path=/; Max-Age=0; SameSite=Strict; Expires=Thu, 01 Jan 1970 00:00:00 GMT;",
        ))
        .json(serde_json::to_string("Logged out successfully")?);
    Ok(api_response)
}

#[get("/me")]
pub async fn me(
    db: Data<SurrealDb>,
    token_factory: Data<TokenFactory>,
    req: HttpRequest,
) -> impl Responder {
    HttpResponseBuilder::new(StatusCode::OK)
}
