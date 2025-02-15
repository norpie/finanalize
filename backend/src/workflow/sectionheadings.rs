use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{models::SurrealDBReport, prelude::*, prompting, tasks::Task};

use std::sync::Arc;

use crate::{db::SurrealDb, llm::LLMApi, scraper::BrowserWrapper, search::SearchEngine};

use super::Job;

#[derive(Debug, Serialize)]
struct SectionHeadingTaskInput {
    current_section: Option<String>,
    user_input: String,
}

#[derive(Debug, Deserialize)]
struct SectionHeadingTaskOutput {
    next_section: Option<String>,
    title: String,
    description: Option<String>,
}

pub struct GenerateSectionHeadingsJob;

#[derive(Debug, Serialize)]
pub struct SectionHeading {
    title: String,
    description: Option<String>,
    order: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SurrealDbSectionHeading {
    pub id: Thing,
    pub title: String,
    pub description: Option<String>,
    pub order: usize,
}

#[async_trait]
impl Job for GenerateSectionHeadingsJob {
    async fn run(
        &self,
        report: &SurrealDBReport,
        db: SurrealDb,
        llm: Arc<dyn LLMApi>,
        _search: Arc<dyn SearchEngine>,
        _browser: BrowserWrapper,
    ) -> Result<()> {
        let mut current_section: Option<String> = None;
        let mut order = 1;

        loop {
            let prompt = prompting::get_prompt(db.clone(), "generate_section_heading".into()).await?;
            let section_heading_task = Task::new(&prompt);
            let section_input = SectionHeadingTaskInput {
                current_section: current_section.clone(),
                user_input: report.user_input.clone(),
            };

            let section_output: SectionHeadingTaskOutput =
                section_heading_task.run(llm.clone(), &section_input).await?;

            // Save the generated heading to the database
            let section_heading = SectionHeading {
                title: section_output.title.clone(),
                description: section_output.description.clone(),
                order,
            };

            let sdb_section: SurrealDbSectionHeading = db
                .create("section_headings")
                .content(section_heading)
                .await?
                .ok_or(FinanalizeError::UnableToCreateSectionHeading)?;

            // Relate section to the report
            db.query("RELATE $report ->has_section -> $section")
                .bind(("report", report.id.clone()))
                .bind(("section", sdb_section.id.clone()))
                .await?;

            // Break the loop if no next section is specified
            if section_output.next_section.is_none() {
                break;
            }

            // Update state for the next iteration
            current_section = section_output.next_section;
            order += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{db, llm::ollama::Ollama, models::ReportCreation, scraper, search::SearxNG};

    use super::*;

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test() {
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
        let job = GenerateSectionHeadingsJob;
        job.run(&report, db, llm, search, browser).await.unwrap();
    }
}
    