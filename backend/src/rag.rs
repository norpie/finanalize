use crate::{llm::LLMApi, prelude::*};

use std::sync::Arc;
use log::debug;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::db::SurrealDb;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub description: String,
    pub r#type: String,
    pub url: String,
}

impl From<SurrealDbDocument> for Document {
    fn from(doc: SurrealDbDocument) -> Self {
        Document {
            id: doc.id.id.to_string(),
            title: doc.title,
            description: doc.description,
            url: doc.url,
            r#type: doc.r#type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealDbDocument {
    pub id: Thing,
    pub title: String,
    pub description: String,
    pub r#type: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    pub content: String,
    pub distance: f64,
    pub report_source: Document,
}

impl From<SurrealDbDocumentChunk> for DocumentChunk {
    fn from(chunk: SurrealDbDocumentChunk) -> Self {
        DocumentChunk {
            content: chunk.content,
            distance: chunk.distance,
            report_source: chunk.report_source.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealDbDocumentChunk {
    pub content: String,
    pub distance: f64,
    pub report_source: SurrealDbDocument,
}

static VECTOR_SEARCH_QUERY: &str = r#"
SELECT content, distance, report_source FROM (SELECT
    content,
    vector::similarity::cosine(embedding, $search_embed) AS distance,
    <-has_content_chunk<-report_source AS report_source
FROM (
    (SELECT
        ->has_research->report_source->has_content_chunk->source_chunk AS chunks
    FROM ONLY
        $report
    FETCH chunks)
).chunks
ORDER BY distance ASC)
SPLIT report_source
FETCH report_source;
"#;

pub async fn vector_search(
    db: Arc<SurrealDb>,
    llm_api: Arc<dyn LLMApi>,
    report: Thing,
    query: String,
) -> Result<Vec<DocumentChunk>> {
    debug!("Searching for '{:#?}'", &query);
    let search_embed = llm_api.embed(query).await?;
    debug!("Embedding length: {}", &search_embed.len());
    let results: Vec<SurrealDbDocumentChunk> = db
        .query(VECTOR_SEARCH_QUERY)
        .bind(("search_embed", search_embed))
        .bind(("report", report))
        .await?
        .take(0)?;
    debug!("Found {} results", results.len());
    Ok(results.into_iter().map(DocumentChunk::from).collect())
}

#[cfg(test)]
mod tests {
    use crate::{db, llm::ollama::Ollama};

    use super::*;
    use surrealdb::sql::Thing;

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test_vector_search() {
        let db = Arc::new(db::connect().await.unwrap());
        let llm_api = Arc::new(Ollama::default());
        let report = Thing::from(("report", "jgq1yy5g4i5zfgv8w1xy"));
        let query = "Hello".to_string();
        let results = vector_search(db, llm_api, report, query).await.unwrap();
        dbg!(&results);
        assert_eq!(results.len(), 3);
    }
}
