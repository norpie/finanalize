use crate::{llm, prelude::*, prompting, tasks::Task};

use async_trait::async_trait;
use models::ValidationOutput;

use crate::workflow::WorkflowState;

use super::Job;

pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ValidationInput {
        pub message: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
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
        let prompt = prompting::get_prompt("validation".into())?;
        let task = Task::new(&prompt);
        let input = models::ValidationInput {
            message: state.state.user_input.clone(),
        };
        let output: ValidationOutput = task.run(llm::API.clone(), &input).await?;
        state.state.validation = Some(output);
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
            state: FullReport {
                id: "sjaudnhcrlas".into(),
                user_input: "Hello, World!".into(),
                status: JobType::Pending,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                validation: None,
                title: None,
            },
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
            state: FullReport {
                id: "sjaudnhcrlas".into(),
                user_input: "Apple stock in 2025".into(),
                status: JobType::Pending,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                validation: None,
                title: None,
            },
        };
        let new_state = job.run(state).await.unwrap();
        assert!(new_state.state.validation.unwrap().valid);
    }
}
