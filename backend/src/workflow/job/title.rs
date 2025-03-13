use crate::{llm::API, prelude::*, prompting, tasks::{Task, TaskResult}, workflow::WorkflowState};

use super::{validation::models::ValidationInput, Job};

use async_trait::async_trait;
use log::debug;
use models::TitleOutput;
use schemars::schema_for;

pub mod models {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
    pub struct TitleOutput {
        pub title: String,
    }
}

pub struct TitleJob;

#[async_trait]
impl Job for TitleJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        debug!("Running TitleJob...");
        let prompt = prompting::get_prompt("title".into())?;
        let task = Task::new(&prompt);
        let input = ValidationInput {
            message: state.state.user_input.clone(),
        };
        debug!("Prepared input: {:#?}", input);
        debug!("Running task...");
        let res: TaskResult<TitleOutput> = task
            .run_structured(
                API.clone(),
                &input,
                serde_json::to_string_pretty(&schema_for!(TitleOutput))?,
            )
            .await?;
        debug!("Task completed");
        state.state.title = Some(res.output.title);
        state.state.generation_results.push(res.info);
        debug!("Title: {:#?}", state.state.title);
        dbg!(&state.state.title);
        debug!("TitleJob completed");
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        models::FullReport,
        workflow::{job::validation::models::ValidationOutput, JobType, WorkflowState},
    };

    #[tokio::test]
    #[ignore = "Uses LLM API (External Service)"]
    async fn test_title_job_valid() {
        env_logger::init();
        let job = TitleJob;
        let state = WorkflowState {
            id: "tlksajbdfaln".into(),
            last_job_type: JobType::Pending,
            state: FullReport::new("sjaudnhcrlas".into(), "Apple stock in 2025".into())
                .with_validation(ValidationOutput {
                    valid: true,
                    error: None,
                }),
        };
        job.run(state).await.unwrap().state.title.unwrap();
    }
}
