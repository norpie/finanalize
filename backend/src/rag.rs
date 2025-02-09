use crate::{llm::LLMApi, prelude::*};

use std::sync::Arc;

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
    pub id: String,
    pub content: String,
    pub distance: f64,
    pub report_source: Document,
}

impl From<SurrealDbDocumentChunk> for DocumentChunk {
    fn from(chunk: SurrealDbDocumentChunk) -> Self {
        DocumentChunk {
            id: chunk.id.id.to_string(),
            content: chunk.content,
            distance: chunk.distance,
            report_source: chunk.report_source.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealDbDocumentChunk {
    pub id: Thing,
    pub content: String,
    pub distance: f64,
    pub report_source: SurrealDbDocument,
}

static VECTOR_SEARCH_QUERY: &str = r#"
SELECT id, content, distance, report_source FROM (SELECT
    content,
    vector::similarity::cosine(embedding, $search_embed) AS distance,
    <-has_content_chunk<-report_source AS report_source
FROM (
    (SELECT
        ->has_research->report_source->has_content_chunk->source_chunk AS chunks
    FROM ONLY
        $report
    GROUP ALL
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
) -> Result<Vec<Document>> {
    let search_embed = llm_api.embed(query).await?;
    let results: Vec<SurrealDbDocument> = db
        .query(VECTOR_SEARCH_QUERY)
        .bind(("search_embed", search_embed))
        .bind(("report", report))
        .await?
        .take(0)?;
    Ok(results.into_iter().map(Document::from).collect())
}
