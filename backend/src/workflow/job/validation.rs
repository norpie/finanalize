use crate::{llm, prelude::*, prompting, tasks::{Task, TaskResult}, workflow::JobType};

use async_trait::async_trait;
use log::debug;
use models::ValidationOutput;
use schemars::schema_for;

use crate::workflow::WorkflowState;

use super::Job;

pub mod models {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
    pub struct ValidationInput {
        pub message: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
    pub struct ValidationOutput {
        pub valid: bool,
        pub error: Option<String>,
    }
}

pub struct ValidationJob;

#[async_trait]
impl Job for ValidationJob {
    /// Expects the previous state to be a `Report`
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        debug!("Running ValidationJob...");
        let prompt = prompting::get_prompt("validation".into())?;
        let task = Task::new(&prompt);
        let input = models::ValidationInput {
            message: state.state.user_input.clone(),
        };
        debug!("Prepared input: {:#?}", input);
        debug!("Running task...");
        let res: TaskResult<ValidationOutput> = task
            .run_structured(
                llm::API.clone(),
                &input,
                serde_json::to_string_pretty(&schema_for!(ValidationOutput))?,
            )
            .await?;
        let output = res.output;
        state.state.generation_results.push(res.info);
        debug!("Task completed");
        state.state.status = JobType::Invalid;
        state.state.validation = Some(output);
        debug!("Validation: {:#?}", state.state.validation);
        dbg!(&state.state.validation);
        debug!("ValidationJob completed");
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        models::FullReport,
        workflow::{JobType, WorkflowState},
    };

    #[tokio::test]
    #[ignore = "Uses LLM API (External Service)"]
    async fn test_validation_job_invalid() {
        let job = ValidationJob;
        let state = WorkflowState {
            id: "tlksajbdfaln".into(),
            last_job_type: JobType::Pending,
            state: FullReport::new("sjaudnhcrlas".into(), "Hello, World!".into()),
        };
        let new_state = job.run(state).await.unwrap();
        assert!(!new_state.state.validation.unwrap().valid);
    }

    #[tokio::test]
    #[ignore = "Uses LLM API (External Service)"]
    async fn test_validation_job_valid() {
        let job = ValidationJob;
        let state = WorkflowState {
            id: "tlksajbdfaln".into(),
            last_job_type: JobType::Pending,
            state: FullReport::new("sjaudnhcrlas".into(), "Apple stock in 2025".into()),
        };
        let new_state = job.run(state).await.unwrap();
        assert!(new_state.state.validation.unwrap().valid);
    }
}
