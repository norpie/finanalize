use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::sql::Thing;

use super::Job;
use crate::{
    db::SurrealDb, llm::LLMApi, models::SurrealDBReport, prelude::*, prompting,
    scraper::BrowserWrapper, search::SearchEngine, tasks::Task,
};

#[derive(Debug, Serialize)]
struct SectionHeadingTaskInput {
    heading: Option<String>,
    message: String,
    title: String, // Add the title field if required by the API
}

#[derive(Debug, Deserialize)]
struct SectionHeadingTaskOutput {
    next_section: Option<String>,
    headings: Option<Vec<Heading>>, // Expect an array of headings
}

#[derive(Debug, Deserialize)]
struct Heading {
    heading: String,
    description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SectionHeading {
    heading: String,
    description: Option<String>,
    order: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SurrealDbSectionHeading {
    pub id: Thing,
    pub heading: String,
    pub description: Option<String>,
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
        _browser: BrowserWrapper,
    ) -> Result<()> {
        // Fetch prompt and prepare the input
        let prompt = prompting::get_prompt(db.clone(), "heading".into()).await?;
        println!("Generated Prompt: {}", prompt); // Log the generated prompt for debugging

        let task = Task::new(&prompt);

        // Create task input (with title included)
        let input = SectionHeadingTaskInput {
            heading: None, // Adjust if initial heading exists
            message: report.user_input.clone(),
            title: "Financial performance analysis of Apple and Microsoft in Q4 2024".to_string(), // Example title, update as needed
        };

        // Log the input JSON to ensure it's well-formed
        println!(
            "Serialized Input JSON: {}",
            serde_json::to_string(&input).unwrap()
        );

        // Run task to generate sections
        let output: SectionHeadingTaskOutput = task.run(llm.clone(), &input).await?;

        // Log the raw API response for debugging
        println!("Raw API response: {:?}", output);

        // Check if the 'headings' field is present and handle accordingly
        if let Some(headings) = output.headings {
            if !headings.is_empty() {
                // Process each heading in the response
                for (index, heading) in headings.iter().enumerate() {
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
            } else {
                eprintln!("API response 'headings' field is empty.");
            }
        } else {
            eprintln!("Missing 'headings' field in the API response.");
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
    #[ignore = "Depends on external services"]
    async fn test_generate_section_headings() {
        env::set_var("OLLAMA_BASE_URL", "http://10.147.17.202:11434");
        let db = db::connect().await.unwrap();
        let llm = Arc::new(Ollama::default());
        let search = Arc::new(SearxNG::new("http://localhost:8081"));
        scraper::setup_browser().await.unwrap();
        let browser = scraper::INSTANCE.get().unwrap().clone();

        let creation = ReportCreation::new("Test Report Title".into());
        let report: SurrealDBReport = db
            .create("report")
            .content(creation)
            .await
            .unwrap()
            .unwrap();

        let job = GenerateSectionHeadingsJob;
        job.run(&report, db, llm, search, browser).await.unwrap();
    }
}
