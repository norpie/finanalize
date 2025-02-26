use log::debug;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tokio::sync::OnceCell;

use crate::prelude::*;

pub static DB: OnceCell<SurrealDb> = OnceCell::const_new();

pub type SurrealDb = Surreal<Client>;

pub async fn connect() -> Result<SurrealDb> {
    let mut default_address = "localhost:8000".to_string();
    if let Ok(env_address) = std::env::var("SURREALDB_URL") {
        default_address = env_address;
        debug!("Using database at: {}", default_address);
    }
    let db = Surreal::new::<Ws>(default_address).await?;
    debug!("Connected to database");
    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("finanalize").use_db("db").await?;
    debug!("Ready to use database");
    Ok(db)
}
