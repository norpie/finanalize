use super::Job;
use crate::db::SurrealDb;
use crate::llm::LLMApi;
use crate::models::SurrealDBReport;
use crate::prelude::*;
use crate::scraper::{scrape_page, BrowserWrapper};
use crate::search::SearchEngine;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::sql::Thing;

pub struct ScrapeTopResultsJob;

#[derive(Debug, Serialize, Clone)]
struct ScrapedContent {
    content: String
}
#[derive(Debug, Deserialize, Clone)]
struct SurrealDBScrapedContent {
    id: Thing,
    content: String
}

#[derive(Debug, Deserialize, Clone)]
struct SurrealDBSourceURL {
    id: Thing,
    url: String
}

#[async_trait]
impl Job for ScrapeTopResultsJob {
    async fn run(
        &self,
        report: &SurrealDBReport,
        db: SurrealDb,
        _llm: Arc<dyn LLMApi>,
        _search: Arc<dyn SearchEngine>,
        _browser: BrowserWrapper,
    ) -> Result<()> {
        // Get the source url's from the database, and scrape them using the custom scraper.rs
        let mut db_source_urls = db
            .query("SELECT ->has_source_url->source_url as source_urls FROM $report FETCH source_urls")
            .bind(("report", report.id.clone()))
            .await?;
        let sdb_source_urls: Vec<SurrealDBSourceURL> = db_source_urls
            .take::<Option<Vec<SurrealDBSourceURL>>>("source_urls")?
            .ok_or(FinanalizeError::NotFound)?;

        // Scrape the top results from the source urls
        for sdb_url in sdb_source_urls {
            let scraped_content = ScrapedContent{
                content: scrape_page(sdb_url.url).await?
            };
            // Save the scraped data to the database
            let sdb_scraped_content: SurrealDBScrapedContent = db
                .create("scraped_content")
                .content(scraped_content)
                .await?
                .ok_or(FinanalizeError::UnableToAddScrapedUrl)?;
            db.query("RELATE $source_url ->has_scraped_content-> $scraped_content")
                .bind(("source_url", sdb_url.id.clone()))
                .bind(("scraped_content", sdb_scraped_content.id.clone()))
                .await?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use serde::Serialize;
    use super::*;
    use crate::llm::ollama::Ollama;
    use crate::models::{ReportCreation, SurrealDBReport};
    use crate::search::SearxNG;
    use crate::{db, scraper};

    #[derive(Debug, Deserialize, Serialize)]
    struct TestURL{
        url: String,
    }
    #[derive(Debug, Deserialize, Serialize)]
    struct TestSDBurl{
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
        scraper::setup_browser().await.unwrap();
        let browser = scraper::INSTANCE.get().unwrap().clone();
        let creation = ReportCreation::new("Apple 2025 Q4 outlook".into());
        let report: SurrealDBReport = db
            .create("report")
            .content(creation)
            .await
            .unwrap()
            .unwrap();
        dbg!(&report);
        let urls: Vec<String> =
           vec![
              "https://en.wikipedia.org/wiki/Example.com".into(),
              "https://www.example.com".into(),
          ];

        for url in urls{
            let db_url = TestURL {
                url: url.clone(),
            };
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
        ScrapeTopResultsJob.run(&report, db, llm, search, browser).await.unwrap();
    }
}