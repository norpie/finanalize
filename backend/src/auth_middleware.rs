use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use log::debug;
use crate::jwt::TokenFactory;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Clone)]
pub struct Auth {
    token_factory: TokenFactory,
}

impl Auth {
    pub fn new(token_factory: TokenFactory) -> Self {
        debug!("Initializing Auth Middleware...");
        Self { token_factory }
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        debug!("Creating AuthMiddleware instance...");
        ready(Ok(AuthMiddleware {
            service,
            token_factory: self.token_factory.clone(),
        }))
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    service: S,
    token_factory: TokenFactory,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        debug!("AuthMiddleware processing request: {:?}", req.path());
        let Some(token) = get_token(&req) else {
            debug!("No Authorization token found. Rejecting request.");
            return Box::pin(async {
                Ok(req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body()))
            });
        };
        debug!("Extracted token: {:?}", token);
        let Ok(sub) = self.token_factory.subject(token) else {
            debug!("Invalid or expired token. Rejecting request.");
            return Box::pin(async {
                Ok(req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body()))
            });
        };
        debug!("Token successfully verified. Injecting subject into request.");
        req.extensions_mut().insert(sub);
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            debug!("Token successfully verified. Injecting subject into request.");
            Ok(res.map_into_left_body())
        })
    }
}

fn get_token(req: &ServiceRequest) -> Option<&str> {
    debug!("Extracting authentication token...");
    req.headers()
        .get("Authorization")?
        .to_str()
        .ok()?
        .split_whitespace()
        .last()
}
