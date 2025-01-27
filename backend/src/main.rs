use crate::prelude::*;
use actix_cors::Cors;
use actix_web::{
    web::{self, Data},
    App, HttpServer, Responder,
};
use api::{
    v1::auth::{login, logout, me, refresh, register},
    ApiResponse,
};
use jwt::TokenFactory;

mod api;
mod db;
mod jwt;
mod models;
mod prelude;
mod search;

#[tokio::main]
async fn main() -> Result<()> {
    let db = db::connect().await?;
    let token_factory: TokenFactory = "secret".into();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_, _| true)
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(Data::new(token_factory.clone()))
            .app_data(Data::new(db.clone()))
            .default_service(web::route().to(not_found))
            .service(
                web::scope("/api/v1/auth")
                    .service(login)
                    .service(logout)
                    .service(me)
                    .service(refresh)
                    .service(register),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}

async fn not_found() -> impl Responder {
    ApiResponse::<()>::error(404, FinanalizeError::NotFound.to_string())
}
