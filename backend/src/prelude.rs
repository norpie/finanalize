use derive_more::derive::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, FinanalizeError>;

#[derive(Debug, Error)]
pub enum FinanalizeError {
    #[error("Not found")]
    NotFound,
    #[error("Authorization error: {0}")]
    Unauthorized(AuthError),
    #[error("Not implemented")]
    NotImplemented,
    #[error("Internal server error")]
    InternalServerError,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("SurrealDB error: {0}")]
    SurrealDB(#[from] surrealdb::Error),
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("Argon2 error: {0}")]
    Argon2(#[from] argon2::password_hash::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, Display)]
pub enum AuthError {
    #[display("Invalid token")]
    InvalidToken,
    #[display("Expired token")]
    ExpiredToken,
    #[display("Invalid refresh token")]
    MissingCredentials,
    #[display("Invalid credentials")]
    InvalidCredentials,
    #[display("Email already exists")]
    EmailAlreadyExists,
    #[display("Missing token")]
    MissingToken,
}
