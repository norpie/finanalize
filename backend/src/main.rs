use crate::prelude::*;
use api::v1::auth::{login, register, refresh, logout, me};

mod search;
mod db;
mod prelude;
mod api;

#[tokio::main]
async fn main() -> Result<()> {
    let db = db::connect().await?;
    let mut app = tide::new();
    app.at("/api/v1/auth/login").post(login);
    app.at("/api/v1/auth/register").post(register);
    app.at("/api/v1/auth/refresh").post(refresh);
    app.at("/api/v1/auth/logout").post(logout);
    app.at("/api/v1/auth/me").get(me);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
