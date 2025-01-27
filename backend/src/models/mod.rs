use serde::{Deserialize, Serialize};
use surrealdb::Uuid;
use surrealize::Surrealize;

#[derive(Debug, Clone, Serialize, Deserialize, Surrealize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}
