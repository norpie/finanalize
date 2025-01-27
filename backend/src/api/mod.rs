use crate::prelude::*;
use actix_web::{body::BoxBody, Responder};
use serde::{Deserialize, Serialize};

use crate::{AuthError, FinanalizeError};

pub mod v1;

static DEFAULT_ERROR: &str = "{ \"error\": \"Internal server error\" }";

#[derive(Debug, Serialize, Deserialize)]
pub enum UserError {
    InvalidToken,
    ExpiredToken,
    InvalidCredentials,
    InternalServerError,
    NotImplemented,
    NotFound
}

impl From<FinanalizeError> for UserError {
    fn from(e: FinanalizeError) -> Self {
        match e {
            FinanalizeError::Unauthorized(AuthError::InvalidToken) => UserError::InvalidToken,
            FinanalizeError::Unauthorized(AuthError::ExpiredToken) => UserError::ExpiredToken,
            FinanalizeError::Unauthorized(AuthError::InvalidCredentials) => {
                UserError::InvalidCredentials
            }
            FinanalizeError::NotImplemented => UserError::NotImplemented,
            FinanalizeError::NotFound => UserError::NotFound,
            _ => UserError::InternalServerError,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiResult<T> {
    #[serde(rename = "result")]
    Result(T),
    #[serde(rename = "error")]
    Error(UserError),
}

impl<T: Serialize> ApiResult<T> {
    pub fn new(obj: T) -> Self {
        ApiResult::Result(obj)
    }

    pub fn error(err: FinanalizeError) -> Self {
        ApiResult::Error(err.into())
    }
}

impl<T: Serialize> From<Result<T>> for ApiResult<T> {
    fn from(result: Result<T>) -> Self {
        match result {
            Ok(obj) => ApiResult::Result(obj),
            Err(e) => ApiResult::Error(e.into()),
        }
    }
}

impl<T: Serialize> Responder for ApiResult<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let ser_result = serde_json::to_string(&self);
        match ser_result {
            Ok(json) => actix_web::HttpResponse::Ok().body(json),
            Err(_) => actix_web::HttpResponse::InternalServerError().body(DEFAULT_ERROR),
        }
    }
}
