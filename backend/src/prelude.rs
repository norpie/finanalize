use derive_more::derive::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tide::{Response};

pub type Result<T> = std::result::Result<T, FinanalizeError>;

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiResult<T> {
    Ok(T),
    Err(UserError)
}

static DEFAULT_ERROR: &str = "{ \"error\": \"Internal server error\" }";

impl<T: Serialize> From<ApiResult<T>> for tide::Result<Response> {
    fn from(value: ApiResult<T>) -> Self {
        let ser_result = serde_json::to_string(&value);
        match ser_result {
            Ok(json) => {
                let builder = Response::builder(200).body(json);
                Ok(builder.build())
            }
            Err(_) => {
                let builder = Response::builder(500).body(DEFAULT_ERROR);
                Ok(builder.build())
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserError {
    InvalidToken,
    ExpiredToken,
    InvalidCredentials,
    InternalServerError,
}

#[derive(Debug, Error)]
pub enum FinanalizeError {
    #[error("Not found")]
    NotFound,
    #[error("Authorization error: {0}")]
    Unauthorized(AuthError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("SurrealDB error: {0}")]
    SurrealDB(#[from] surrealdb::Error),
}

#[derive(Debug, Display)]
pub enum AuthError {
    InvalidToken,
    ExpiredToken,
    InvalidCredentials,
}

impl From<FinanalizeError> for UserError {
    fn from(e: FinanalizeError) -> Self {
        match e {
            FinanalizeError::Unauthorized(AuthError::InvalidToken) => UserError::InvalidToken,
            FinanalizeError::Unauthorized(AuthError::ExpiredToken) => UserError::ExpiredToken,
            FinanalizeError::Unauthorized(AuthError::InvalidCredentials) => UserError::InvalidCredentials,
            _ => UserError::InternalServerError,
        }
    }
}
