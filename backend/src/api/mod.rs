use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    HttpResponse, HttpResponseBuilder, Responder, ResponseError,
};
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
    NotFound,
}

impl From<&FinanalizeError> for UserError {
    fn from(e: &FinanalizeError) -> Self {
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

impl ResponseError for FinanalizeError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            FinanalizeError::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            FinanalizeError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<BoxBody> {
        let user_error: UserError = self.into();
        let json = serde_json::to_string(&user_error).unwrap_or_else(|_| DEFAULT_ERROR.to_string());
        HttpResponseBuilder::new(self.status_code())
            .content_type(ContentType::json())
            .body(json)
    }
}

/// Wrapper for every response made by the backend
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing)]
    status: StatusCode,
    #[serde(skip_serializing)]
    cookie: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    /// Wrap object in ApiResponse
    pub fn new(object: T) -> Self {
        ApiResponse {
            result: Some(object),
            status: StatusCode::OK,
            error: None,
            cookie: None,
        }
    }

    /// Wrap error in ApiResponse
    pub fn error(status: StatusCode, error: String) -> Self {
        ApiResponse {
            error: Some(error),
            status,
            result: None,
            cookie: None,
        }
    }

    pub fn with_cookie(mut self, cookie: String) -> Self {
        self.cookie = Some(cookie);
        self
    }
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let result = serde_json::to_string(&self);
        match result {
            Ok(json) => {
                let mut builder = HttpResponseBuilder::new(self.status);
                builder.content_type(ContentType::json());
                if let Some(cookie) = &self.cookie {
                    builder.insert_header(("Set-Cookie", cookie.as_str()));
                }
                builder.body(json)
            }
            Err(_e) => HttpResponse::InternalServerError().body(""),
        }
    }
}
