use crate::{db::DB, models::FullReport, prelude::*, rabbitmq::PUBLISHER};

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
    let output = process_state(workflow_state).await?;
    if output.state.status.next().is_none() {
        debug!("Workflow for report {} is done", output.id);
        return Ok(());
    }
    // If it's not done, publish the next job
    let saved: SDBWorkflowState = DB
        .get()
        .unwrap()
        .upsert(("workflow_state", &output.id))
        .content(output)
        .await?
        .ok_or(FinanalizeError::NotFound)?;
    debug!("Saved workflow state for report {}", saved.id);
    let publisher = PUBLISHER.get().unwrap();
    publisher
        .channel
        .basic_publish(
            "",
            publisher.queue.name().as_str(),
            BasicPublishOptions::default(),
            serde_json::to_string(&WorkflowState::from(saved))?.as_bytes(),
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
    let Some(next_type) = state.last_job_type.next() else {
        state.state.status = JobType::Done;
        debug!("No more jobs to run for report {}", &state.id);
        return Ok(state);
    };
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
    GenerateTitle,
    GenerateSectionNames,
    GenerateSubSections,
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::Utc;

    use super::*;

    #[tokio::test]
    async fn test_process_state() {
        env_logger::init();
        let mut state = WorkflowState {
            id: "test".to_string(),
            last_job_type: JobType::Pending,
            state: FullReport {
                id: "asdlfjhasldfjh".into(),
                user_input: "Apple in 2025".into(),
                status: JobType::Pending,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                validation: None,
                title: None,
                sections: None,
                sub_sections: None,
                searches: None,
                search_results: None,
                sources: None,
                report: None,
            },
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
    }
}
