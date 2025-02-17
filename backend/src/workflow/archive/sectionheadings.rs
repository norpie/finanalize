use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::sql::Thing;

use super::Job;
use crate::{
    db::SurrealDb, llm::LLMApi, models::SurrealDBReport, prelude::*, prompting,
    search::SearchEngine, tasks::Task, workflow::title::SurrealDBTitle,
};

#[derive(Debug, Serialize)]
struct SectionHeadingTaskInput {
    message: String,
    title: String,
}

#[derive(Debug, Deserialize)]
struct SectionHeadingTaskOutput {
    headings: Vec<Heading>, // Expect an array of headings
}

#[derive(Debug, Deserialize)]
struct Heading {
    heading: String,
    description: String,
}

#[derive(Debug, Serialize)]
pub struct SectionHeading {
    heading: String,
    description: String,
    order: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SurrealDbSectionHeading {
    pub id: Thing,
    pub heading: String,
    pub description: String,
    pub order: usize,
}

pub struct GenerateSectionHeadingsJob;

#[async_trait]
impl Job for GenerateSectionHeadingsJob {
    async fn run(
        &self,
        report: &SurrealDBReport,
        db: SurrealDb,
        llm: Arc<dyn LLMApi>,
        _search: Arc<dyn SearchEngine>,
    ) -> Result<()> {
        // Fetch prompt and prepare the input
        let prompt = prompting::get_prompt(db.clone(), "heading".into()).await?;
        println!("Generated Prompt: {}", prompt); // Log the generated prompt for debugging

        let task = Task::new(&prompt);

        let title: SurrealDBTitle = db
            .query(
                "SELECT * FROM (SELECT ->has_title->report_title as titles FROM $report)[0].titles",
            )
            .bind(("report", report.id.clone()))
            .await?
            .take::<Option<SurrealDBTitle>>(0)?
            .ok_or(FinanalizeError::MissingReportTitle)?;

        // Create task input (with title included)
        let input = SectionHeadingTaskInput {
            message: report.user_input.clone(),
            title: title.title.clone(),
        };

        // Run task to generate sections
        let output: SectionHeadingTaskOutput = task.run(llm.clone(), &input).await?;

        for (index, heading) in output.headings.iter().enumerate() {
            let section_heading = SectionHeading {
                heading: heading.heading.clone(),
                description: heading.description.clone(),
                order: index + 1, // The order depends on the index in the array
            };

            // Create the section heading entity in the database
            let db_section: SurrealDbSectionHeading = db
                .create("section_headings")
                .content(section_heading)
                .await?
                .ok_or(FinanalizeError::UnableToCreateSectionHeading)?;

            // Relate section to the report
            db.query("RELATE $report ->has_section -> $section")
                .bind(("report", report.id.clone()))
                .bind(("section", db_section.id.clone()))
                .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db, llm::ollama::Ollama, models::ReportCreation, search::SearxNG};
    use std::env;

    #[tokio::test]
    #[ignore = "Depends on external services"]
    async fn test_generate_section_headings() {
        env::set_var("OLLAMA_BASE_URL", "http://10.147.17.202:11434");
        let db = db::connect().await.unwrap();
        let llm = Arc::new(Ollama::default());
        let search = Arc::new(SearxNG::new("http://localhost:8081"));

        let creation = ReportCreation::new("Test Report Title".into());
        let report: SurrealDBReport = db
            .create("report")
            .content(creation)
            .await
            .unwrap()
            .unwrap();

        let job = GenerateSectionHeadingsJob;
        job.run(&report, db, llm, search).await.unwrap();
    }
}
