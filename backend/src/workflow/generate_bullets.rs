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

pub struct GenerateBulletsJob;
// Job input and output
#[derive(Debug, Serialize)]
struct GenerateBulletsInput {
    message: String,
    title: Title,
    headings: Vec<Heading>,
}
#[derive(Debug, Serialize, Deserialize)]
struct GenerateBulletsOutput {
    paragraphs: Vec<Paragraph>,
}

// Prompt structs
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Title {
    report_title: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Heading {
    report_section_heading: String,
    description: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Paragraph {
    heading: String,
    paragraph: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SurrealDBParagraph {
    id: Thing,
    heading: String,
    paragraph: Vec<String>,
}

#[async_trait]
impl Job for GenerateBulletsJob {
    async fn run(
        &self,
        report: &SurrealDBReport,
        db: SurrealDb,
        llm: Arc<dyn LLMApi>,
        _search: Arc<dyn SearchEngine>,
        _browser: BrowserWrapper,
    ) -> Result<()> {
        let prompt = prompting::get_prompt(db.clone(), "paragraph".into()).await?;
        let mut db_titles = db
            .query("SELECT * FROM (SELECT ->has_title->title as titles FROM $report FETCH titles)[0].titles[0]")
            .bind(("report", report.id.clone()))
            .await?;
        let mut db_section_headings = db
            .query("SELECT ->has_section_heading->section_headings as headings FROM $report FETCH headings")
            .bind(("report", report.id.clone()))
            .await?;

        let message: String = report.user_input.clone();
        let titles: Vec<Title> = db_titles.take(0)?;
        let title: &Title = titles.first().ok_or(FinanalizeError::NotFound)?;
        let headings: Vec<Heading> = db_section_headings
            .take::<Option<Vec<Heading>>>("headings")?
            .ok_or(FinanalizeError::NotFound)?;

        let gen_bullets_task = Task::new(&prompt);
        let gen_bullets_input = GenerateBulletsInput {
            message,
            title: title.clone(),
            headings,
        };
        let gen_bullets_output: GenerateBulletsOutput =
            gen_bullets_task.run(llm, &gen_bullets_input).await?;
        let gen_paragraphs = gen_bullets_output.paragraphs;
        for paragraph in gen_paragraphs.iter() {
            let sdb_paragraph: SurrealDBParagraph = db
                .create("paragraph")
                .content(paragraph.clone())
                .await?
                .ok_or(FinanalizeError::UnableToGenerateBullets)?;
            db.query("RELATE $report ->has_paragraph -> $paragraph")
                .bind(("report", report.id.clone()))
                .bind(("paragraph", sdb_paragraph.id.clone()))
                .await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::ollama::Ollama;
    use crate::models::{ReportCreation, SurrealDBReport};
    use crate::search::SearxNG;
    use crate::{db, scraper};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestSDBTitle {
        id: Thing,
        report_title: String,
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestTitle {
        report_title: String,
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestSDBHeading {
        id: Thing,
        report_section_heading: String,
        description: String,
    }

    #[tokio::test]
    #[ignore = "Depends on external services"]
    async fn test_generate_bullets() {
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
        // Create the necessary input in the db
        let title = TestTitle {
            report_title: "Apple 2025 Q4 outlook".to_string(),
        };
        let sdb_title: TestSDBTitle = db.create("title").content(title).await.unwrap().unwrap();
        db.query("RELATE $report ->has_title-> $title")
            .bind(("report", report.id.clone()))
            .bind(("title", sdb_title.id.clone()))
            .await
            .unwrap();
        let headings = vec![
            Heading {
                report_section_heading: "Introduction".to_string(),
                description: "A brief introduction to the report.".to_string(),
            },
            Heading {
                report_section_heading: "iPhone".to_string(),
                description: "Information about the new iPhone model.".to_string(),
            },
        ];
        for heading in headings {
            let sdb_heading: TestSDBHeading = db
                .create("section_heading")
                .content(heading)
                .await
                .unwrap()
                .unwrap();
            db.query("RELATE $report ->has_section_heading-> $heading")
                .bind(("report", report.id.clone()))
                .bind(("heading", sdb_heading.id.clone()))
                .await
                .unwrap();
        }

        let job = GenerateBulletsJob;
        job.run(&report, db, llm, search, browser).await.unwrap();
    }
}
