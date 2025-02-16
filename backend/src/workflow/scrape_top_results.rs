use super::Job;
use crate::llm::LLMApi;
use crate::models::SurrealDBReport;
use crate::prelude::*;
use crate::scraper::scrape_page;
use crate::search::SearchEngine;
use crate::{db::SurrealDb, scraper::get_or_init_browser};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::sql::Thing;

pub struct ScrapeTopResultsJob;

#[derive(Debug, Serialize, Clone)]
struct ScrapedContent {
    content: String,
}
#[derive(Debug, Deserialize, Clone)]
struct SurrealDBScrapedContent {
    id: Thing,
    content: String,
}

#[derive(Debug, Deserialize, Clone)]
struct SurrealDBSourceURL {
    id: Thing,
    url: String,
}

#[async_trait]
impl Job for ScrapeTopResultsJob {
    async fn run(
        &self,
        report: &SurrealDBReport,
        db: SurrealDb,
        _llm: Arc<dyn LLMApi>,
        _search: Arc<dyn SearchEngine>,
    ) -> Result<()> {
        // Get the source url's from the database, and scrape them using the custom scraper.rs
        let db_source_urls = db
            .query(
                "SELECT * FROM (SELECT ->has_search_result->search_result as urls FROM $report FETCH urls)[0].urls",
            )
            .bind(("report", report.id.clone()))
            .await?.take::<Vec<SurrealDBSourceURL>>(0)?;
        // Scrape the top results from the source urls
        for sdb_url in db_source_urls {
            let scraped_content = ScrapedContent {
                content: scrape_page(sdb_url.url).await?,
            };
            // Save the scraped data to the database
            let sdb_scraped_content: SurrealDBScrapedContent = db
                .create("scraped_content")
                .content(scraped_content)
                .await?
                .ok_or(FinanalizeError::UnableToAddScrapedUrl)?;
            db.query("RELATE $report ->has_scraped_content-> $scraped_content")
                .bind(("report", report.id.clone()))
                .bind(("scraped_content", sdb_scraped_content.id.clone()))
                .await?;
        }
        get_or_init_browser().await?.close().await?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::ollama::Ollama;
    use crate::models::{ReportCreation, SurrealDBReport};
    use crate::search::SearxNG;
    use crate::db;
    use serde::Serialize;

    #[derive(Debug, Deserialize, Serialize)]
    struct TestURL {
        url: String,
    }
    #[derive(Debug, Deserialize, Serialize)]
    struct TestSDBurl {
        id: Thing,
        url: String,
    }

    #[tokio::test]
    #[ignore = "Depends on external services"]
    async fn test_scrape_top_results() {
        dotenvy::from_filename(".env").ok();
        let db = db::connect().await.unwrap();
        let llm = Arc::new(Ollama::default());
        let search = Arc::new(SearxNG::new("http://localhost:8081"));
        let creation = ReportCreation::new("Apple 2025 Q4 outlook".into());
        let report: SurrealDBReport = db
            .create("report")
            .content(creation)
            .await
            .unwrap()
            .unwrap();
        dbg!(&report);
        let urls: Vec<String> = vec![
            "https://en.wikipedia.org/wiki/Example.com".into(),
            "https://www.example.com".into(),
        ];

        for url in urls {
            let db_url = TestURL { url: url.clone() };
            let db_url: TestSDBurl = db
                .create("source_url")
                .content(db_url)
                .await
                .unwrap()
                .unwrap();
            db.query("RELATE $report ->has_source_url-> $source_url")
                .bind(("report", report.id.clone()))
                .bind(("source_url", db_url.id.clone()))
                .await
                .unwrap();
        }
        ScrapeTopResultsJob
            .run(&report, db, llm, search)
            .await
            .unwrap();
    }
}
