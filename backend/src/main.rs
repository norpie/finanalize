use std::sync::Arc;

use crate::prelude::*;
use actix_cors::Cors;
use actix_web::{
    http::StatusCode,
    web::{self, Data},
    App, HttpServer, Responder,
};
use api::{
    v1::auth::{login, logout, me, refresh, register},
    v1::report::{create_report, get_report, get_reports},
    ApiResponse,
};
use auth_middleware::Auth;
use jwt::TokenFactory;
use llm::{ullm::UllmApi, LLMApi};

mod api;
mod auth_middleware;
mod db;
mod jwt;
mod llm;
mod models;
mod prelude;
mod rabbitmq;
mod search;
mod tasks;
mod scraper;

#[tokio::main]
async fn main() -> Result<()> {
    let db = db::connect().await?;
    let token_factory: TokenFactory = "secret".into();
    let llm: Arc<dyn LLMApi> = Arc::new(UllmApi::default());
    scraper::setup_browser().await?;

    // Initialize the RabbitMQ consumer background task
    tokio::spawn(async move {
        let consumer = rabbitmq::RabbitMQConsumer::new().await?;
        consumer.consume_report_status().await
    });
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_, _| true)
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        let auth_middleware = Auth::new(token_factory.clone());

        App::new()
            .wrap(cors)
            .app_data(Data::new(token_factory.clone()))
            .app_data(Data::new(db.clone()))
            .default_service(web::route().to(not_found))
            .service(
                web::scope("/api/v1/auth")
                    .service(login)
                    .service(register)
                    .service(refresh),
            )
            .service(
                web::scope("/api/v1/protected")
                    .wrap(auth_middleware.clone())
                    .service(logout)
                    .service(me)
                    .service(create_report)
                    .service(get_report)
                    .service(get_reports),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}

async fn not_found() -> impl Responder {
    ApiResponse::<()>::error(StatusCode::NOT_FOUND, FinanalizeError::NotFound.to_string())
}
