use actix_web::Error;
use derive_more::derive::Display;
use fantoccini::error::{CmdError, NewSessionError};
use selectors::parser::SelectorParseErrorKind;
use thiserror::Error;
use scraper::error::SelectorErrorKind;

pub type StdResult<T, E> = std::result::Result<T, E>;
pub type Result<T> = std::result::Result<T, FinanalizeError>;

#[derive(Debug, Error)]
pub enum FinanalizeError {
    #[error("Not found")]
    NotFound,
    #[error("Authorization error: {0}")]
    Unauthorized(AuthError),
    // #[error("Not implemented")]
    // NotImplemented,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Missing prompt file: {0}")]
    MissingPromptFile(String),
    #[error("Missing prompt UTF-8: {0}")]
    MissingPromptUTF8(String),

    #[error("Some retry logic generated the following errors: {0:#?}")]
    MultipleErrors(Vec<FinanalizeError>),

    #[error("LLM API error: {0}")]
    LlmApi(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Env error: {0}")]
    Env(#[from] std::env::VarError),
    #[error("FromUtf8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Tokio error: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("Fantoccini error: {0}")]
    FantocciniCmd(String),
    #[error("Fantoccini error: {0}")]
    FantocciniNewSession(String),

    #[error("SurrealDB error: {0}")]
    SurrealDB(#[from] surrealdb::Error),
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("Argon2 error: {0}")]
    Argon2(#[from] argon2::password_hash::Error),
    #[error("RabbitMQ error: {0}")]
    RabbitMQ(#[from] lapin::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Handlebars error: {0}")]
    Handlebars(#[from] handlebars::RenderError),
    #[error("Polars error: {0}")]
    Polars(#[from] polars::prelude::PolarsError),
    #[error("Xlsl error: {0}")]
    Excel(#[from] calamine::XlsxError),
    #[error("Lopdf error: {0}")]
    LopdfError(#[from] lopdf::Error),
    #[error("Deadpool error: {0}")]
    PoolError(#[from] deadpool::unmanaged::PoolError),
    #[error("pdf_extract error: {0}")]
    Output(#[from] pdf_extract::OutputError),
    #[error("Actix Websocket error: {0}")]
    Websocket(String),
}

impl From<actix_web::Error> for FinanalizeError {
    fn from(value: Error) -> Self {
        Self::Websocket(value.to_string())
    }
}

// #[error("Fantoccini error: {0}")]
// FantocciniCmd(#[from] fantoccini::error::CmdError),
// #[error("Fantoccini error: {0}")]
// FantocciniNewSession(#[from] fantoccini::error::NewSessionError),
impl From<CmdError> for FinanalizeError {
    fn from(error: CmdError) -> Self {
        FinanalizeError::FantocciniCmd(format!("{:?}", error)) // Convert to FinanalizeError::FantocciniCmd
    }
}

impl From<NewSessionError> for FinanalizeError {
    fn from(error: NewSessionError) -> Self {
        FinanalizeError::FantocciniNewSession(format!("{:?}", error)) // Convert to FinanalizeError::FantocciniNewSession
    }
}

impl<'a> From<SelectorParseErrorKind<'a>> for FinanalizeError {
    fn from(error: SelectorParseErrorKind<'a>) -> Self {
        FinanalizeError::ParseError(format!("{:?}", error)) // Convert to FinanalizeError::ParseError
    }
}
impl<'a> From<SelectorErrorKind<'a>> for FinanalizeError {
    fn from(error: SelectorErrorKind<'a>) -> Self {
        // Wrap `SelectorErrorKind` in your `ParseError` variant
        FinanalizeError::ParseError(format!("{:?}", error))
    }
}

#[derive(Debug, Display)]
pub enum AuthError {
    #[display("Invalid token")]
    InvalidToken,
    // #[display("Expired token")]
    // ExpiredToken,
    // #[display("Invalid refresh token")]
    // MissingCredentials,
    #[display("Invalid credentials")]
    InvalidCredentials,
    #[display("Email already exists")]
    EmailAlreadyExists,
    // #[display("Missing token")]
    // MissingToken,
}
