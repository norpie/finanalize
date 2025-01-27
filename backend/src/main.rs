use crate::prelude::*;
use actix_web::{
    web::{self, Data},
    App, HttpServer, Responder,
};
use api::{
    v1::auth::{login, logout, me, refresh, register},
    ApiResult,
};

mod api;
mod db;
mod models;
mod prelude;
mod search;

#[tokio::main]
async fn main() -> Result<()> {
    let db = db::connect().await?;
    HttpServer::new(move || {
        App::new()
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
    ApiResult::<()>::error(FinanalizeError::NotFound)
}
