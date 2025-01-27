use derive_more::derive::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, FinanalizeError>;

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiResult<T> {
    Ok(T),
    Err(UserError)
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
