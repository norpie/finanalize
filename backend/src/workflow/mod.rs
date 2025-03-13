use crate::{
    db::DB, llm::GenerationResult, models::{FullReport, SurrealDBReport}, prelude::*, rabbitmq::PUBLISHER
};

use job::validation::models::ValidationOutput;
use lapin::{message::Delivery, options::BasicPublishOptions, BasicProperties, Channel};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowState {
    pub id: String,
    pub last_job_type: JobType,
    pub state: FullReport,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SDBWorkflowState {
    pub id: Thing,
    pub last_job_type: JobType,
    pub state: FullReport,
}

impl From<SDBWorkflowState> for WorkflowState {
    fn from(sdb: SDBWorkflowState) -> Self {
        Self {
            id: sdb.id.id.to_string(),
            last_job_type: sdb.last_job_type,
            state: sdb.state,
        }
    }
}

pub mod job;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StatusUpdate {
    status: JobType,
    generation_results: Vec<GenerationResult>
}

pub async fn consume_report_status(channel: &Channel, delivery: &Delivery) -> Result<()> {
    let workflow_state: WorkflowState = serde_json::from_slice(&delivery.data)?;
    if workflow_state.last_job_type.is_end_condition() {
        channel
            .basic_ack(delivery.delivery_tag, Default::default())
            .await?;
        return Ok(());
    }
    let next_type = workflow_state.last_job_type.next().unwrap();
    let next_job = next_type.job().unwrap();
    debug!(
        "Running job {:?} for report {}",
        next_type, &workflow_state.id
    );
    let res = next_job.run(workflow_state.clone()).await;
    if let Err(err) = res {
        error!(
            "Error running job {:?} for report {}: {:?}",
            next_type, &workflow_state.id, &err
        );
        let mut tbs_clone = workflow_state.clone();
        tbs_clone.state.sources = None;
        tbs_clone.state.md_sources = None;
        tbs_clone.state.html_sources = None;
        tbs_clone.state.chunks = None;
        tbs_clone.state.chunk_embeddings = None;
        tbs_clone.state.status = JobType::Failed;
        tbs_clone.state.validation = Some(ValidationOutput {
            valid: false,
            error: Some("Failed while generating.".to_string()),
        });
        let _saved: SDBWorkflowState = DB
            .get()
            .unwrap()
            .upsert(("workflow_state", &workflow_state.id))
            .content(tbs_clone)
            .await?
            .ok_or(FinanalizeError::NotFound)?;
        channel
            .basic_nack(delivery.delivery_tag, Default::default())
            .await?;

        return Ok(());
    };
    let mut output = res.unwrap();
    debug!("Job {:?} for report {} completed", next_type, output.id);
    output.last_job_type = next_type;
    output.state.status = next_type.next().unwrap();
    let mut tbs_clone = output.clone();
    tbs_clone.state.sources = None;
    tbs_clone.state.md_sources = None;
    tbs_clone.state.html_sources = None;
    tbs_clone.state.chunks = None;
    tbs_clone.state.chunk_embeddings = None;
    let saved: SDBWorkflowState = DB
        .get()
        .unwrap()
        .upsert(("workflow_state", &output.id))
        .content(tbs_clone)
        .await?
        .ok_or(FinanalizeError::NotFound)?;
    debug!("Saved workflow state for report {}", &saved.id);
    let _new_report_state: SurrealDBReport = DB
        .get()
        .unwrap()
        .update(("report", saved.id.id.to_string().as_str()))
        .merge(StatusUpdate {
            status: output.state.status,
            generation_results: output.state.generation_results.clone()
        })
        .await?
        .ok_or(FinanalizeError::NotFound)?;
    // debug!("Updated report state for report {:#?}", &new_report_state);
    if output.state.status.is_end_condition() {
        debug!("Workflow for report {} is done", output.id);
        channel
            .basic_ack(delivery.delivery_tag, Default::default())
            .await?;
        return Ok(());
    }
    // If it's not done, publish the next job
    let publisher = PUBLISHER.get().unwrap();
    publisher
        .channel
        .basic_publish(
            "",
            publisher.queue.name().as_str(),
            BasicPublishOptions::default(),
            serde_json::to_string(&output)?.as_bytes(),
            BasicProperties::default(),
        )
        .await?;
    // Acknowledge the message
    channel
        .basic_ack(delivery.delivery_tag, Default::default())
        .await?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobType {
    // Starting state
    Pending,
    // Main workflow
    Validation,

    // Generate the report title
    GenerateTitle,
    // Generate the section names
    GenerateSectionNames,
    // Generate the subsections
    GenerateSubSectionNames,
    // Generate questions to be answered in each subsection
    GenerateSubSectionQuestions,
    // Generate search queries to answer the questions
    GenerateSearchQueries,
    // Search the queries
    SearchQueries,
    // Scrape the results of the searches
    ScrapeTopResults,
    // Extract the content of the scraped pages
    ExtractContent,
    // Extract the data from the scraped content
    // ExtractData,
    // Format and summarize the content
    FormatContent,
    // Classify the content
    ClassifyContent,
    // Classify the data
    // ClassifyData,
    // Generate visualizations from the data
    // GenerateVisualizations,
    // Make the literal images of the graphs
    // GenerateGraphs,
    // Chunk content
    ChunkContent,
    // Index the chunks
    IndexChunks,
    // Answer the questions with RAG
    AnswerQuestions,
    // Convert the question and answers into subsection conbtent
    SectionizeQuestions,
    // Put the graphs in the right places
    // RenderGraphs,
    // Put all the content in the template, render it, then compile it to a PDF
    RenderLaTeXPdf,
    // Generate a preview of the PDF
    GeneratePreviewDocument,
    // The three end conditions
    Invalid,
    Done,
    Failed,
}

impl JobType {
    pub fn is_end_condition(&self) -> bool {
        matches!(self, JobType::Invalid | JobType::Done | JobType::Failed)
    }
}
