use super::Job;
use crate::db::SurrealDb;
use crate::llm::LLMApi;
use crate::models::SurrealDBReport;
use crate::prelude::*;
use crate::scraper::{scrape_page, BrowserWrapper};
use crate::search::SearchEngine;
use serde::Deserialize;
use std::sync::Arc;
use async_trait::async_trait;
use surrealdb::sql::Thing;

pub struct ScrapeTopResultsJob;

#[derive(Debug, Deserialize, Clone)]
struct SurrealDBScrapedUrl {
    id: Thing,
    url: String,
}
#[async_trait]
impl Job for ScrapeTopResultsJob {
    async fn run(
        &self,
        report: &SurrealDBReport,
        db: Arc<SurrealDb>,
        _llm: Arc<dyn LLMApi>,
        _search: Arc<dyn SearchEngine>,
        _browser: BrowserWrapper,
    ) -> Result<()> {
        // Get the source url's from the database, and scrape them using the custom scraper.rs
        let mut db_source_urls = db
            .query("SELECT ->has_source_urls->source_urls FROM $report")
            .bind(("report", report.id.clone()))
            .await?;
        let source_urls: Vec<String> = db_source_urls
            .take::<Option<Vec<String>>>(0)?
            .ok_or(FinanalizeError::NotFound)?;
        
        // Scrape the top results from the source urls
        for url in source_urls {
            let scraped_url = scrape_page(url).await?;
            // Save the scraped data to the database
            let sdb_scraped_url: SurrealDBScrapedUrl = db
                .create("scraped_url")
                .content(scraped_url)
                .await?
                .ok_or(FinanalizeError::UnableToAddScrapedUrl)?;
            db.query("RELATE $report ->has_scraped_urls -> $scraped_url")
                .bind(("report", report.id.clone()))
                .bind(("scraped_url", sdb_scraped_url.id.clone()))
                .await?;
        }
        Ok(())
    }
}
