use derive_more::derive::Display;
use selectors::parser::SelectorParseErrorKind;
use thiserror::Error;

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
    #[error("Missing prompt: {0}")]
    MissingPrompt(String),
    #[error("Trying to perform job on non-existing report")]
    ReportNotFound,
    #[error("Unable to create report verdict")]
    UnableToCreateReportVerdict,
    #[error("Unable to update report")]
    UnableToUpdateReport,
    #[error("Unable to create report title")]
    UnableToCreateReportTitle,
    #[error("Unable to create bullets for heading")]
    UnableToGenerateBullets,
    #[error("Unable to add scraped url")]
    UnableToAddScrapedUrl,
    #[error("Unable to create heading url")]
    UnableToCreateSectionHeading,
    #[error("Unable to save search query")]
    UnableToSaveSearchQuery,

    #[error("LLM API error: {0}")]
    LlmApi(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Env error: {0}")]
    Env(#[from] std::env::VarError),
    #[error("FromUtf8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Tokio error: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("Fantoccini error: {0}")]
    FantocciniCmd(#[from] fantoccini::error::CmdError),
    #[error("Fantoccini error: {0}")]
    FantocciniNewSession(#[from] fantoccini::error::NewSessionError),

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
    #[error("Task execution error: {0}")]
    TaskExecutionError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Missing report title")]
    MissingReportTitle,
}

impl From<SelectorParseErrorKind<'_>> for FinanalizeError {
    fn from(error: SelectorParseErrorKind<'_>) -> Self {
        // Manually format the error as a string using `Debug`
        FinanalizeError::ParseError(format!("{:?}", error)) // Convert to FinanalizeError::ParseError
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
