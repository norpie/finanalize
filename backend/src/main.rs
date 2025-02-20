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
use db::DB;
use jwt::TokenFactory;
use rabbitmq::RabbitMQPublisher;

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
    env_logger::init();

    let token_factory: TokenFactory = "secret".into();

    let db = db::connect().await?;
    DB.set(db.clone()).unwrap();

    RabbitMQPublisher::setup().await?;

    // Initialize the RabbiAtMQ consumer background task
    tokio::spawn(async move {
        rabbitmq::RabbitMQConsumer::new()
            .await?
            .consume_report_status()
            .await
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
