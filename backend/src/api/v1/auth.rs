use actix_web::{get, post, Responder};

use crate::{api::ApiResult, FinanalizeError};

#[post("/refresh")]
pub async fn refresh() -> impl Responder {
    // TODO: Implement refresh
    ApiResult::<()>::error(FinanalizeError::NotImplemented)
}

#[post("/login")]
pub async fn login() -> impl Responder {
    // TODO: Implement login
    ApiResult::<()>::error(FinanalizeError::NotImplemented)
}

#[post("/register")]
pub async fn register() -> impl Responder {
    // TODO: Implement register
    ApiResult::<()>::error(FinanalizeError::NotImplemented)
}

#[post("/logout")]
pub async fn logout() -> impl Responder {
    // TODO: Implement logout
    ApiResult::<()>::error(FinanalizeError::NotImplemented)
}

#[get("/me")]
pub async fn me() -> impl Responder {
    // TODO: Implement me
    ApiResult::<()>::error(FinanalizeError::NotImplemented)
}
