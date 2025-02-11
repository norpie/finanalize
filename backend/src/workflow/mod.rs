//! ```mermaid
//! %%{init: {'theme': 'neutral', 'themeVariables': { 'primaryColor': '#e6f3ff'}}}%%
//! graph TD
//!     A[User Input] --> B{Valid Input?}
//!     B -->|Yes| C[Generate Report Title]
//!     B -->|No| Z[Error: Invalid Input]
//!     C --> D[Generate Section Headings]
//!     D --> E[[For Each Heading]]
//!     E --> F[Generate Bullet Points]
//!     F --> G[[For Each Bullet Point]]
//!     G --> H[Generate Search Queries]
//!     H --> I[[For Each Query]]
//!     I --> J[Scrape Top 5 Results]
//!     J --> K[[Process Results]]
//!     K --> L[Extract Structured Data]
//!     K --> M[Extract Unstructured Content]
//!     L --> N[Annotate Data Sources]
//!     M --> O[Annotate Content Sources]
//!     N --> P[RAG Processing]
//!     O --> P
//!     P --> Q[Generate Text Chunks]
//!     Q --> R[Combine into Coherent Paragraph]
//!     R --> S[Assemble Section Content]
//!     S --> T[[Add Citations]]
//!     T --> U[Identify Visualization Needs]
//!     U --> V[Generate/Pull Visualizations]
//!     V --> W[Finalize Section]
//!     W --> X[[Compile All Sections]]
//!     X --> Y[Generate PDF Report]
//!
//!     style A fill:#4CAF50,color:white
//!     style B fill:#FFC107,color:black
//!     style Z fill:#F44336,color:white
//!     style Y fill:#2196F3,color:white
//!     classDef loop fill:#fff8e1,stroke:#ffb300;
//!     class E,G,I,K loop;
//! ```
//!
//! A workflows describes the sequence of jobs that need to happen to complete
//! the main goal. In our case we only have one workflow, "generate report".
//! Which takes in the `user_input` and generates a financial report for it. The
//! above mermaid diagram describes our "workflow", consisting of all the "jobs"
//! that need to be done to generate a report.
//!
//! This file contains the data structures that represent the workflow and the
//! jobs.
use std::sync::Arc;

use crate::{
    db::SurrealDb,
    llm::LLMApi,
    models::{ReportStatus, SurrealDBReport},
    prelude::*,
    scraper::BrowserWrapper,
    search::SearchEngine,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub async fn run_next_job(
    report_id: String,
    db: Arc<SurrealDb>,
    llm: Arc<dyn LLMApi>,
    search: Arc<dyn SearchEngine>,
    browser: BrowserWrapper,
) -> Result<()> {
    let mut report: SurrealDBReport = db
        .select(("report", report_id))
        .await?
        .ok_or(FinanalizeError::ReportNotFound)?;
    let job_type = JobType::from(&report.status);
    let job = job_type.job();
    job.run(&report, db.clone(), llm, search, browser).await?;
    let Some(next) = job_type.next() else {
        return Ok(());
    };
    report.status = next.into();
    let report: SurrealDBReport = db
        .update(("report", report.id.id.to_string()))
        .content(report)
        .await?
        .ok_or(FinanalizeError::UnableToUpdateReport)?;
    dbg!(report);
    Ok(())
}

#[async_trait]
pub trait Job {
    /// Runs the job.
    ///
    /// # Arguments
    /// - `report_id` - The ID of the report that the job is being run for.
    /// - `db` - The database connection.
    /// - `llm` - The LLM API connection.
    /// - `search` - The search engine connection.
    /// - `browser` - The browser connection.
    ///
    /// # Returns
    /// - `Ok(())` if the job was successful.
    /// - `Err(Error)` if the job failed.
    async fn run(
        &self,
        report: &SurrealDBReport,
        db: Arc<SurrealDb>,
        llm: Arc<dyn LLMApi>,
        search: Arc<dyn SearchEngine>,
        browser: BrowserWrapper,
    ) -> Result<()>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobType {
    Pending,
    Validation,
    GenerateTitle,
    GenerateSectionHeadings,
    GenerateParagraphBullets,
    GenerateSearchQueries,
    SearchQueries,
    ScrapeTopResults,
    ExtractContent,
    ExtractStructuredData,
    ChunkText,
    RAGPrepareChunks,
    GenerateBulletTexts,
    CombineBulletsIntoParagraph,
    AssembleSectionContent,
    AddCitations,
    IdentifyVisualizationNeeds,
    GenerateVisualizations,
    FinalizeSection,
    CompileSections,
    GeneratePDFReport,
    Done,
}

impl From<&ReportStatus> for JobType {
    fn from(value: &ReportStatus) -> Self {
        serde_json::from_value(serde_json::to_value(value).unwrap()).unwrap()
    }
}

impl From<JobType> for ReportStatus {
    fn from(value: JobType) -> Self {
        serde_json::from_value(serde_json::to_value(value).unwrap()).unwrap()
    }
}

impl JobType {
    /// Advances the job type to the next step in the workflow.
    ///
    /// # Returns
    /// - `Some(JobType)` if the current state is not `Done`
    /// - `None` if the current state is `Done`, as there are no more steps.
    pub fn next(&self) -> Option<JobType> {
        match self {
            JobType::Pending => Some(JobType::Validation),
            JobType::Validation => Some(JobType::GenerateTitle),
            JobType::GenerateTitle => Some(JobType::GenerateSectionHeadings),
            JobType::GenerateSectionHeadings => Some(JobType::GenerateParagraphBullets),
            JobType::GenerateParagraphBullets => Some(JobType::GenerateSearchQueries),
            JobType::GenerateSearchQueries => Some(JobType::SearchQueries),
            JobType::SearchQueries => Some(JobType::ScrapeTopResults),
            JobType::ScrapeTopResults => Some(JobType::ExtractContent),
            JobType::ExtractContent => Some(JobType::ExtractStructuredData),
            JobType::ExtractStructuredData => Some(JobType::ChunkText),
            JobType::ChunkText => Some(JobType::RAGPrepareChunks),
            JobType::RAGPrepareChunks => Some(JobType::GenerateBulletTexts),
            JobType::GenerateBulletTexts => Some(JobType::CombineBulletsIntoParagraph),
            JobType::CombineBulletsIntoParagraph => Some(JobType::AssembleSectionContent),
            JobType::AssembleSectionContent => Some(JobType::AddCitations),
            JobType::AddCitations => Some(JobType::IdentifyVisualizationNeeds),
            JobType::IdentifyVisualizationNeeds => Some(JobType::GenerateVisualizations),
            JobType::GenerateVisualizations => Some(JobType::FinalizeSection),
            JobType::FinalizeSection => Some(JobType::CompileSections),
            JobType::CompileSections => Some(JobType::GeneratePDFReport),
            JobType::GeneratePDFReport => Some(JobType::Done),
            JobType::Done => None,
        }
    }

    pub fn job(&self) -> Box<dyn Job> {
        match self {
            JobType::Validation => Box::new(validation::ValidationJob),
            _ => Box::new(nop::NopJob),
        }
    }
}

mod nop {
    use std::sync::Arc;

    use async_trait::async_trait;

    use crate::{
        db::SurrealDb, llm::LLMApi, models::SurrealDBReport, prelude::*, scraper::BrowserWrapper,
        search::SearchEngine,
    };

    use super::Job;

    pub struct NopJob;

    #[async_trait]
    impl Job for NopJob {
        async fn run(
            &self,
            _report: &SurrealDBReport,
            _db: Arc<SurrealDb>,
            _llm: Arc<dyn LLMApi>,
            _search: Arc<dyn SearchEngine>,
            _browser: BrowserWrapper,
        ) -> Result<()> {
            Ok(())
        }
    }
}

mod title;
mod validation;
mod bullets_gen;
