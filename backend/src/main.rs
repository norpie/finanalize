use std::sync::Arc;

use crate::prelude::*;
use actix_cors::Cors;
use actix_web::{
    http::StatusCode,
    web::{self, Data},
    App, HttpServer, Responder,
};
use api::{
    v1::{
        auth::{login, logout, me, refresh, register},
        report::{create_report, get_report, get_reports, retry},
    },
    ApiResponse,
};
use auth_middleware::Auth;
use jwt::TokenFactory;
use llm::{ollama::Ollama, LLMApi};
use rabbitmq::RabbitMQPublisher;
use search::SearxNG;

mod api;
mod auth_middleware;
mod db;
#[allow(dead_code)]
mod extractors;
mod jwt;
#[allow(dead_code)]
mod llm;
mod models;
mod prelude;
#[allow(dead_code)]
mod prompting;
mod rabbitmq;
#[allow(dead_code)]
mod rag;
#[allow(dead_code)]
mod scraper;
#[allow(dead_code)]
mod search;
#[allow(dead_code)]
mod sec;
#[allow(dead_code)]
mod tasks;
#[allow(dead_code)]
mod workflow;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::from_filename(".env").ok();
    let db = db::connect().await?;
    let token_factory: TokenFactory = "secret".into();
    let llm: Arc<dyn LLMApi> = Arc::new(Ollama::default());
    let search = Arc::new(SearxNG::new("http://localhost:8081"));
    RabbitMQPublisher::setup().await?;
    let db_clone = db.clone();

    // Initialize the RabbiAtMQ consumer background task
    tokio::spawn(async move {
        let db = db.clone();
        let llm = llm.clone();
        let search = search.clone();
        let consumer = rabbitmq::RabbitMQConsumer::new().await?;
        consumer.consume_report_status(db, llm, search).await
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
            .app_data(Data::new(db_clone.clone()))
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
                    .service(retry)
                    .service(get_report)
                    .service(get_reports),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}

async fn not_found() -> impl Responder {
    ApiResponse::<()>::error(StatusCode::NOT_FOUND, FinanalizeError::NotFound.to_string())
}
