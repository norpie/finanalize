use crate::{api::ApiResponse, db::SurrealDb, models::SurrealDBUser, prelude::*};

use crate::{models::ChangesetUser, FinanalizeError};
use actix_web::{
    get, post,
    web::{Data, Json},
};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

#[post("/refresh")]
pub async fn refresh() -> Result<ApiResponse<()>> {
    // TODO: Implement refresh
    Ok(ApiResponse::error(
        501,
        FinanalizeError::NotImplemented.to_string(),
    ))
}

#[post("/login")]
pub async fn login() -> Result<ApiResponse<()>> {
    // TODO: Implement login
    Ok(ApiResponse::error(
        501,
        FinanalizeError::NotImplemented.to_string(),
    ))
    // verify password against PHC string
    // let parsed_hash = PasswordHash::new(&hash)?;
    // assert!(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok());
}

#[post("/register")]
pub async fn register(
    db: Data<SurrealDb>,
    user: Json<ChangesetUser>,
) -> Result<ApiResponse<SurrealDBUser>> {
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
    let hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    user.password = Some(hash);
    Ok(ApiResponse::new(
        db.create("user").content(user).await?.unwrap(),
    ))
}

#[post("/logout")]
pub async fn logout() -> Result<ApiResponse<()>> {
    // TODO: Implement logout
    Ok(ApiResponse::error(
        501,
        FinanalizeError::NotImplemented.to_string(),
    ))
}

#[get("/me")]
pub async fn me() -> Result<ApiResponse<()>> {
    // TODO: Implement me
    Ok(ApiResponse::error(
        501,
        FinanalizeError::NotImplemented.to_string(),
    ))
}
