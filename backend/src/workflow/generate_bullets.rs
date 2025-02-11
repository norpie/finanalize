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

pub struct BulletsGenJob;
// Job input and output
#[derive(Debug, Serialize)]
struct BulletsGenInput {
    message: String,
    title: String,
    headings: Vec<Heading>,
}
#[derive(Debug, Serialize, Deserialize)]
struct BulletsGenOutput {
    paragraphs: Vec<Paragraph>,
}

// Prompt structs
#[derive(Debug, Serialize, Deserialize)]
struct Heading {
    heading: String,
    description: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Paragraph {
    heading: String,
    has_bullets: Vec<Bullet>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Bullet {
    bullet: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealDBParagraphs {
    id: Thing,
    has_bullets: Vec<Paragraph>,
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
        let mut db_message = db
            .query("SELECT ->has_message->report_message FROM $report")
            .bind(("report", report.clone()))
            .await?;
        let mut db_title = db
            .query("SELECT ->has_title->report_title FROM $report")
            .bind(("report", report.clone()))
            .await?;
        let mut db_section_headings = db
            .query("SELECT ->has_section_heading->report_section_heading FROM $report")
            .bind(("report", report.clone()))
            .await?;

        let message: String = db_message.take::<Option<String>>(0)?.unwrap();
        let title: String = db_title.take::<Option<String>>(0)?.unwrap();
        let headings: Vec<Heading> = db_section_headings
            .take::<Option<Vec<Heading>>>(0)?
            .unwrap();

        let gen_bullets_task = Task::new(&prompt);
        let gen_bullets_input = BulletsGenInput {
            message,
            title,
            headings,
        };
        let gen_bullets_output: BulletsGenOutput =
            gen_bullets_task.run(llm, &gen_bullets_input).await?;

        let sdb_paragraphs: SurrealDBParagraphs = db
            .create("paragraphs")
            .content(gen_bullets_output.paragraphs)
            .await?
            .ok_or(FinanalizeError::UnableToGenerateBullets)?;
        db.query("RELATE $report -> has_paragraphs -> $paragraphs")
            .bind(("report", report.id.clone()))
            .bind(("paragraphs", sdb_paragraphs.id.clone()))
            .await?;

        Ok(())
    }
}
