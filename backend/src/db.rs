use async_lazy::Lazy;
use log::debug;
use surrealdb::{
    engine::remote::ws::{self, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::prelude::*;

pub static DB: Lazy<SurrealDb> =
    Lazy::new(|| Box::pin(async { connect().await.expect("Failed to connect to database") }));

pub type SurrealDb = Surreal<ws::Client>;

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
