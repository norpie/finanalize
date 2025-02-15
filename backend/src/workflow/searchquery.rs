use super::Job;
use crate::{db::SurrealDb, llm::LLMApi, scraper::BrowserWrapper, search::SearchEngine};
use crate::{models::SurrealDBReport, prelude::*};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::sql::Thing;

#[derive(Debug, Serialize)]
struct SearchResult {
    url: String,
}

#[derive(Debug, Deserialize)]
struct SDBSearchResult {
    id: Thing,
    url: String,
}

#[derive(Debug, Serialize)]
struct SearchQuery {
    query: String,
}
#[derive(Debug, Deserialize)]
struct SDBSearchQuery {
    id: Thing,
    query: String,
}

pub struct SearchGenerationJob;

#[async_trait]
impl Job for SearchGenerationJob {
    async fn run(
        &self,
        report: &SurrealDBReport,
        db: SurrealDb,
        _llm: Arc<dyn LLMApi>,
        search: Arc<dyn SearchEngine>,
        _browser: BrowserWrapper,
    ) -> Result<()> {
        // Step 1: Retrieve search queries
        let queries: Vec<SDBSearchQuery> = db
            .query("SELECT * FROM (SELECT ->has_search_query->search_query as queries FROM $report)[0].queries;")
            .bind(("report", report.id.clone()))
            .await?
            .take(0)?;

        // Step 2: Launch each query and retrieve top 5 results
        for query in queries {
            let results = search.search(&query.query).await?;
            let results = results.into_iter().take(5).collect::<Vec<_>>(); // Limit to 5

            // Step 3: Store each result in the database
            for result in results {
                let search_result = SearchResult { url: result };

                let created: SDBSearchResult = db
                    .create("search_result")
                    .content(search_result)
                    .await
                    .unwrap()
                    .unwrap();
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db, llm::ollama::Ollama, models::ReportCreation, scraper, search::SearxNG};
    use std::env;

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test_search_generation() {
        env::set_var("OLLAMA_BASE_URL", "http://10.147.17.202:11434");
        let db = db::connect().await.unwrap();
        let search = Arc::new(SearxNG::new("http://localhost:8081"));
        scraper::setup_browser().await.unwrap();
        let browser = scraper::INSTANCE.get().unwrap().clone();
        let creation = ReportCreation::new("Apple 2025 Q4 outlook".into());
        let report: SurrealDBReport = db
            .create("report")
            .content(creation)
            .await
            .unwrap()
            .unwrap();

        // Create test search queries
        let queries = vec![
            SearchQuery {
                query: "Apple stock performance".to_string(),
            },
            SearchQuery {
                query: "Best gaming laptops 2025".to_string(),
            },
        ];

        for query in queries {
            let query: SDBSearchQuery = db
                .create("search_query")
                .content(query)
                .await
                .unwrap()
                .unwrap();
            db.query("RELATE $report -> has_search_query -> $query")
                .bind(("report", report.id.clone()))
                .bind(("query", query.id.clone()))
                .await
                .unwrap();
        }

        // Run the job
        let job = SearchGenerationJob;
        job.run(
            &report,
            db.clone(),
            Arc::new(Ollama::default()),
            search,
            browser,
        )
        .await
        .unwrap();
    }
}
