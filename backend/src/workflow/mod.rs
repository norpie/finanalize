use crate::{db::DB_HTTP, models::FullReport, prelude::*, rabbitmq::PUBLISHER};

use lapin::{message::Delivery, options::BasicPublishOptions, BasicProperties, Channel};
use log::debug;
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

pub async fn consume_report_status(channel: &Channel, delivery: &Delivery) -> Result<()> {
    let workflow_state: WorkflowState = serde_json::from_slice(&delivery.data)?;
    let output = process_state(workflow_state.clone()).await?;
    // If it's not done, publish the next job
    let mut tbs_clone = output.clone();
    tbs_clone.state.sources = None;
    tbs_clone.state.md_sources = None;
    tbs_clone.state.html_sources = None;
    tbs_clone.state.chunks = None;
    tbs_clone.state.chunk_embeddings = None;
    let saved: SDBWorkflowState = DB_HTTP
        .get()
        .unwrap()
        .upsert(("workflow_state", &output.id))
        .content(tbs_clone)
        .await?
        .ok_or(FinanalizeError::NotFound)?;
    if output.state.status.next().is_none() {
        debug!("Workflow for report {} is done", output.id);
        return Ok(());
    }
    debug!("Saved workflow state for report {}", saved.id);
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

async fn process_state(mut state: WorkflowState) -> Result<WorkflowState> {
    if state.state.status == JobType::Done {
        debug!("report is already done {}", &state.id);
        return Ok(state);
    }
    let Some(next_type) = state.last_job_type.next() else {
        state.state.status = JobType::Done;
        debug!("No more jobs to run for report {}", &state.id);
        return Ok(state);
    };
    if next_type == JobType::Done {
        state.state.status = JobType::Done;
        debug!("No more jobs to run for report {}", &state.id);
        return Ok(state);
    }
    let Some(next_job) = next_type.job() else {
        state.state.status = JobType::Invalid;
        debug!("No job for type {:?}", next_type);
        return Ok(state);
    };
    debug!("Running job {:?} for report {}", next_type, &state.id);
    let mut output = next_job.run(state).await?;
    debug!("Job {:?} for report {} completed", next_type, output.id);
    output.last_job_type = next_type;
    output.state.status = next_type;
    Ok(output)
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
    // Format and summarize the content
    FormatContent,
    // Classify the content
    ClassifyContent,
    // Chunk content
    ChunkContent,
    // Index the chunks
    IndexChunks,
    // Answer the questions with RAG
    AnswerQuestions,
    // Put all the content in the template, render it, then compile it to a PDF
    RenderLaTeXPdf,
    // The two end conditions
    Invalid,
    Done,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use job::index_chunks::models::EmbeddedChunk;

    use crate::db::{self, DB};

    use super::*;

    #[tokio::test]
    #[ignore = "Depends on external services"]
    async fn test_process_state() {
        env_logger::init();
        DB.set(db::connect().await.unwrap()).unwrap();
        let mut state = WorkflowState {
            id: "asdlfjhasldfjh".to_string(),
            last_job_type: JobType::Pending,
            state: FullReport::new("asdlfjhasldfjh".into(), "Apple in 2025".into()),
        };
        loop {
            println!("{:?}", state.state.status);
            state = process_state(state).await.unwrap();
            if state.state.status.next().is_none() {
                break;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        println!("{:?}", state.state.status);
        let _deleted: SDBWorkflowState = DB
            .get()
            .unwrap()
            .delete(("workflow_state", &state.id))
            .await
            .unwrap()
            .unwrap();
        let _deleted: Vec<EmbeddedChunk> = DB
            .get()
            .unwrap()
            .query("DELETE embedded_chunk WHERE report_id = $id")
            .bind(("id", state.id))
            .await
            .unwrap()
            .take(0)
            .unwrap();
    }
}
