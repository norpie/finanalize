use crate::{llm::API, prelude::*, prompting, tasks::Task, workflow::WorkflowState};

use super::{validation::models::ValidationInput, Job};

use async_trait::async_trait;
use models::TitleOutput;

pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct TitleOutput {
        pub title: String,
    }
}

pub struct TitleJob;

#[async_trait]
impl Job for TitleJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let prompt = prompting::get_prompt("title".into())?;
        let task = Task::new(&prompt);
        let input = ValidationInput {
            message: state.state.user_input.clone(),
        };
        let output: TitleOutput = task.run(API.clone(), &input).await?;
        state.state.title = Some(output.title);
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
    async fn test_title_job_valid() {
        env_logger::init();
        let job = TitleJob;
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
                sections: None,
                sub_sections: None,
            },
        };
        job.run(state).await.unwrap().state.title.unwrap();
    }
}
