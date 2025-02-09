use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::prelude::*;
pub type SurrealDb = Surreal<Client>;

pub async fn connect() -> Result<SurrealDb> {
    let mut default_address = "localhost:8000".to_string();
    if let Ok(env_address) = std::env::var("SURREALDB_URL") {
        default_address = env_address;
    }
    let db = Surreal::new::<Ws>(default_address).await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("finanalize").use_db("db").await?;

    Ok(db)
}
