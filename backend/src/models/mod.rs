use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendUser {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealDBUser {
    pub id: Thing,
    pub email: String,
    pub password: String,
}

impl From<SurrealDBUser> for User {
    fn from(user: SurrealDBUser) -> Self {
        User {
            id: user.id.id.to_string(),
            email: user.email,
            password: user.password,
        }
    }
}

impl From<SurrealDBUser> for FrontendUser {
    fn from(user: SurrealDBUser) -> Self {
        FrontendUser {
            id: user.id.id.to_string(),
            email: user.email,
        }
    }
}
