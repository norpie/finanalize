use crate::{db::DB, llm::API, prelude::*};

use log::debug;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistancedChunk {
    pub report_id: String,
    pub source_id: String,
    pub chunk: String,
    pub distance: f64,
}

static VECTOR_SEARCH_QUERY: &str = r#"
SELECT report_id, source_id, chunk, vector::similarity::cosine(embeddings, $embedding) AS distance FROM embedded_chunk WHERE report_id = $report_id LIMIT 20;
"#;

pub async fn vector_search(report: Thing, query: String) -> Result<Vec<DistancedChunk>> {
    debug!("Searching for '{:#?}'", &query);
    let search_embed = API.clone().embed(query).await?;
    debug!("Embedding length: {}", &search_embed.len());
    let results: Vec<DistancedChunk> = DB
        .get()
        .unwrap()
        .query(VECTOR_SEARCH_QUERY)
        .bind(("embedding", search_embed))
        .bind(("report_id", report.id.to_string()))
        .await?
        .take(0)?;
    debug!("Found {} results", results.len());
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use surrealdb::sql::Thing;

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test_vector_search() {
        let report = Thing::from(("report", "sjaudnhcrlas"));
        let query = "Hello".to_string();
        let results = vector_search(report, query).await.unwrap();
        dbg!(&results);
        assert_eq!(results.len(), 3);
    }
}
