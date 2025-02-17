use std::sync::Arc;

use crate::{
    db::SurrealDb, llm::LLMApi, models::SurrealDBReport, prelude::*, rabbitmq::PUBLISHER,
    search::SearchEngine,
};

use async_trait::async_trait;
use lapin::{message::Delivery, options::BasicPublishOptions, BasicProperties, Channel};
use log::info;
use serde::{Deserialize, Serialize};

//mod generate_bullets;
//mod generate_search_queries;
//mod scrape_top_results;
//mod searchquery;
//mod sectionheadings;
//mod title;
//mod validation;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowStatusUpdate {
    pub report_id: String,
    pub last_job_type: JobType,
    pub last_job_output_json: String,
}

mod job;

pub async fn consume_report_status(channel: &Channel, delivery: &Delivery) -> Result<()> {
    let workflow_status_update: WorkflowStatusUpdate = serde_json::from_slice(&delivery.data)?;
    // TODO: Run next job
    let Some(next_type) = workflow_status_update.last_job_type.next() else {
        info!("No more jobs to run for report {}", workflow_status_update.report_id);
        return Ok(());
    };
    let next_job = next_type.job();
    // TODO: Check if workflow is done after the job

    // If it's not done, publish the next job
    let publisher = PUBLISHER.get().unwrap();
    publisher
        .channel
        .basic_publish(
            "",
            publisher.queue.name().as_str(),
            BasicPublishOptions::default(),
            serde_json::to_string(&workflow_status_update)?.as_bytes(),
            BasicProperties::default(),
        )
        .await?;
    // Acknowledge the message
    channel
        .basic_ack(delivery.delivery_tag, Default::default())
        .await?;
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
    // The two end conditions
    Invalid,
    Done,
}

// impl JobType {
//     /// Advances the job type to the next step in the workflow.
//     ///
//     /// # Returns
//     /// - `Some(JobType)` if the current state is not `Done`
//     /// - `None` if the current state is `Done`, as there are no more steps.
//     pub fn next(&self) -> Option<JobType> {
//         match self {
//             JobType::Pending => Some(JobType::Validation),
//             JobType::Validation => Some(JobType::GenerateTitle),
//             JobType::GenerateTitle => Some(JobType::GenerateSectionHeadings),
//             JobType::GenerateSectionHeadings => Some(JobType::GenerateParagraphBullets),
//             JobType::GenerateParagraphBullets => Some(JobType::GenerateSearchQueries),
//             JobType::GenerateSearchQueries => Some(JobType::SearchQueries),
//             JobType::SearchQueries => Some(JobType::ScrapeTopResults),
//             JobType::ScrapeTopResults => Some(JobType::Done),
//             // ReportStatus::ScrapeTopResults => Some(ReportStatus::ExtractContent),
//             JobType::ExtractContent => Some(JobType::ExtractStructuredData),
//             JobType::ExtractStructuredData => Some(JobType::ChunkText),
//             JobType::ChunkText => Some(JobType::RAGPrepareChunks),
//             JobType::RAGPrepareChunks => Some(JobType::GenerateBulletTexts),
//             JobType::GenerateBulletTexts => Some(JobType::CombineBulletsIntoParagraph),
//             JobType::CombineBulletsIntoParagraph => Some(JobType::AssembleSectionContent),
//             JobType::AssembleSectionContent => Some(JobType::AddCitations),
//             JobType::AddCitations => Some(JobType::IdentifyVisualizationNeeds),
//             JobType::IdentifyVisualizationNeeds => Some(JobType::GenerateVisualizations),
//             JobType::GenerateVisualizations => Some(JobType::FinalizeSection),
//             JobType::FinalizeSection => Some(JobType::CompileSections),
//             JobType::CompileSections => Some(JobType::GeneratePDFReport),
//             JobType::GeneratePDFReport => Some(JobType::Done),
//             JobType::Invalid => None,
//             JobType::Done => None,
//         }
//     }
//
//     pub fn job(&self) -> Box<dyn Job> {
//         match self {
//             JobType::Pending => Box::new(nop::NopJob),
//             JobType::Validation => Box::new(validation::ValidationJob),
//             JobType::GenerateTitle => Box::new(title::TitleJob),
//             JobType::GenerateSectionHeadings => {
//                 Box::new(sectionheadings::GenerateSectionHeadingsJob)
//             }
//             JobType::GenerateParagraphBullets => Box::new(generate_bullets::GenerateBulletsJob),
//             JobType::GenerateSearchQueries => {
//                 Box::new(generate_search_queries::SearchGenerationJob)
//             }
//             JobType::SearchQueries => Box::new(searchquery::SearchQueriesJob),
//             JobType::ScrapeTopResults => Box::new(scrape_top_results::ScrapeTopResultsJob),
//             _ => Box::new(nop::NopJob),
//         }
//     }
// }
