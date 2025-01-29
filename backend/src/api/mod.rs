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
    status: u16,
}

impl<T: Serialize> ApiResponse<T> {
    /// Wrap object in ApiResponse
    pub fn new(object: T) -> Self {
        ApiResponse {
            result: Some(object),
            status: 200,
            error: None,
        }
    }

    /// Wrap error in ApiResponse
    pub fn error(status: u16, error: String) -> Self {
        ApiResponse {
            error: Some(error),
            status,
            result: None,
        }
    }
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let result = serde_json::to_string(&self);
        match result {
            Ok(json) => HttpResponseBuilder::new(StatusCode::from_u16(self.status).unwrap())
                .content_type(ContentType::json())
                .body(json),
            Err(_e) => HttpResponse::InternalServerError().body(""),
        }
    }
}
