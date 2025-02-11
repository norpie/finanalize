use super::Job;
use crate::db::SurrealDb;
use crate::llm::LLMApi;
use crate::models::SurrealDBReport;
use crate::prelude::*;
use crate::prompting;
use crate::scraper::BrowserWrapper;
use crate::search::SearchEngine;
use crate::tasks::Task;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::sql::Thing;

#[derive(Debug, Serialize)]
struct BulletsGenInput {
    title: String,
    headings: Vec<Heading>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Heading {
    heading: String,
    description: String,
}
#[derive(Debug, Deserialize)]
struct BulletsGenOutput {
    paragraphs: Vec<Vec<String>>,
}

pub struct BulletsGenJob;

#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealDBBullets {
    id: Thing,
    paragraphs: Vec<Vec<String>>,
}

#[async_trait]
impl Job for BulletsGenJob {
    async fn run(
        &self,
        report: &SurrealDBReport,
        db: Arc<SurrealDb>,
        llm: Arc<dyn LLMApi>,
        _search: Arc<dyn SearchEngine>,
        _browser: BrowserWrapper,
    ) -> Result<()> {
        let prompt = prompting::get_prompt(db.clone(), "paragraph".into()).await?;
        let mut db_title = db
            .query("SELECT ->has_title->report_title FROM $report")
            .bind(("report", report.clone()))
            .await?;
        let mut db_section_headings = db
            .query("SELECT ->has_section_heading->report_section_heading FROM $report")
            .bind(("report", report.clone()))
            .await?;

        let title: String = db_title.take::<Option<String>>(0)?.unwrap();
        let headings: Vec<Heading> = db_section_headings.take::<Option<Vec<Heading>>>(0)?.unwrap();

        let gen_bullets_task = Task::new(&prompt);
        let gen_bullets_input = BulletsGenInput { title, headings };
        let gen_bullets_output: BulletsGenOutput =
            gen_bullets_task.run(llm, &gen_bullets_input).await?;

        let sdb_bullets: SurrealDBBullets = db
            .create("bullets")
            .content(gen_bullets_output.paragraphs)
            .await?
            .ok_or(FinanalizeError::UnableToGenerateBullets)?;
        db.query("RELATE $report -> has_bullets -> $bullets")
            .bind(("report", report.id.clone()))
            .bind(("bullets", sdb_bullets.id))
            .await?;
        Ok(())
    }
}
