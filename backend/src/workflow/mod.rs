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
    db::SurrealDb, llm::LLMApi, models::SurrealDBReport, prelude::*, scraper::BrowserWrapper,
    search::SearchEngine,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub async fn run_next_job(
    report_id: &String,
    db: SurrealDb,
    llm: Arc<dyn LLMApi>,
    search: Arc<dyn SearchEngine>,
    browser: BrowserWrapper,
) -> Result<()> {
    let mut report: SurrealDBReport = db
        .select(("report", report_id))
        .await?
        .ok_or(FinanalizeError::ReportNotFound)?;
    println!("Running job for report: {}", report.id.id);
    let status = report.status;
    println!("Current status: {:?}", status);
    let job = status.job();
    println!("Running job: {:?}", status);
    job.run(&report, db.clone(), llm, search, browser).await?;
    println!("Job completed successfully");
    let Some(next) = status.next() else {
        println!("No more jobs to run");
        return Ok(());
    };
    report.status = next;
    println!("Updating report status to: {:?}", next);
    let _report: SurrealDBReport = db
        .update(("report", report.id.id.to_string()))
        .content(report)
        .await?
        .ok_or(FinanalizeError::UnableToUpdateReport)?;
    println!("Report updated successfully");
    Ok(())
}

#[async_trait]
pub trait Job: Send + Sync + 'static {
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
        db: SurrealDb,
        llm: Arc<dyn LLMApi>,
        search: Arc<dyn SearchEngine>,
        browser: BrowserWrapper,
    ) -> Result<()>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportStatus {
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
    // The two end conditions
    Invalid,
    Done,
}

impl ReportStatus {
    /// Advances the job type to the next step in the workflow.
    ///
    /// # Returns
    /// - `Some(JobType)` if the current state is not `Done`
    /// - `None` if the current state is `Done`, as there are no more steps.
    pub fn next(&self) -> Option<ReportStatus> {
        match self {
            ReportStatus::Pending => Some(ReportStatus::Validation),
            ReportStatus::Validation => Some(ReportStatus::GenerateTitle),
            ReportStatus::GenerateTitle => Some(ReportStatus::GenerateSectionHeadings),
            ReportStatus::GenerateSectionHeadings => Some(ReportStatus::GenerateParagraphBullets),
            ReportStatus::GenerateParagraphBullets => Some(ReportStatus::GenerateSearchQueries),
            ReportStatus::GenerateSearchQueries => Some(ReportStatus::SearchQueries),
            ReportStatus::SearchQueries => Some(ReportStatus::ScrapeTopResults),
            ReportStatus::ScrapeTopResults => Some(ReportStatus::ExtractContent),
            ReportStatus::ExtractContent => Some(ReportStatus::ExtractStructuredData),
            ReportStatus::ExtractStructuredData => Some(ReportStatus::ChunkText),
            ReportStatus::ChunkText => Some(ReportStatus::RAGPrepareChunks),
            ReportStatus::RAGPrepareChunks => Some(ReportStatus::GenerateBulletTexts),
            ReportStatus::GenerateBulletTexts => Some(ReportStatus::CombineBulletsIntoParagraph),
            ReportStatus::CombineBulletsIntoParagraph => Some(ReportStatus::AssembleSectionContent),
            ReportStatus::AssembleSectionContent => Some(ReportStatus::AddCitations),
            ReportStatus::AddCitations => Some(ReportStatus::IdentifyVisualizationNeeds),
            ReportStatus::IdentifyVisualizationNeeds => Some(ReportStatus::GenerateVisualizations),
            ReportStatus::GenerateVisualizations => Some(ReportStatus::FinalizeSection),
            ReportStatus::FinalizeSection => Some(ReportStatus::CompileSections),
            ReportStatus::CompileSections => Some(ReportStatus::GeneratePDFReport),
            ReportStatus::GeneratePDFReport => Some(ReportStatus::Done),
            ReportStatus::Invalid => None,
            ReportStatus::Done => None,
        }
    }

    pub fn job(&self) -> Box<dyn Job> {
        match self {
            ReportStatus::Pending => Box::new(validation::ValidationJob),
            ReportStatus::Validation => Box::new(title::TitleJob),
            ReportStatus::GenerateTitle => Box::new(nop::NopJob),
            ReportStatus::GenerateSectionHeadings => Box::new(generate_bullets::GenerateBulletsJob),
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
            _db: SurrealDb,
            _llm: Arc<dyn LLMApi>,
            _search: Arc<dyn SearchEngine>,
            _browser: BrowserWrapper,
        ) -> Result<()> {
            Ok(())
        }
    }
}

mod generate_bullets;
mod scrape_top_results;
mod title;
mod validation;
mod sectionheadings;
