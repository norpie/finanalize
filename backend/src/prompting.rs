use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{db::SurrealDb, prelude::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealDbPrompt {
    id: Thing,
    template: String,
}

pub async fn get_prompt(db: SurrealDb, id: String) -> Result<String> {
    let result: SurrealDbPrompt = db
        .select(("prompt", &id))
        .await?
        .ok_or(FinanalizeError::MissingPrompt(id))?;
    Ok(result.template)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{self};

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test_get_prompt() {
        let db = db::connect().await.unwrap();
        let id = ("prompt", "test");
        let mut prompt = SurrealDbPrompt {
            id: Thing::from(id),
            template: "Hello, World!".to_string(),
        };
        prompt = db
            .upsert(id)
            .content(prompt.clone())
            .await
            .unwrap()
            .unwrap();
        let result = get_prompt(db.clone(), "test".into()).await.unwrap();
        assert_eq!(result, prompt.template);
    }
}
