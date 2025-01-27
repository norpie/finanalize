use surrealdb::{engine::remote::ws::{Client, Ws}, opt::auth::Root, Surreal};

use crate::prelude::*;

pub async fn connect() -> Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;

    // Select a specific namespace / database
    db.use_ns("namespace").use_db("database").await?;

    Ok(db)
}
