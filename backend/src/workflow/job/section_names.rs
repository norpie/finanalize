use crate::{llm::API, prelude::*, prompting, tasks::Task};

use async_trait::async_trait;
use log::debug;
use models::{SectionNamesInput, SectionNamesOutput};
use schemars::schema_for;

use crate::workflow::WorkflowState;

use super::Job;

pub mod models {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct SectionNamesInput {
        pub amount: u64,
        pub title: String,
        pub message: String,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
    pub struct SectionNamesOutput {
        pub sections: Vec<String>,
    }
}

pub struct SectionNamesJob;

#[async_trait]
impl Job for SectionNamesJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        debug!("Running SectionNamesJob...");
        let prompt = prompting::get_prompt("section".into())?;
        let task = Task::new(&prompt);
        let input = SectionNamesInput {
            amount: state.state.size.section_amount(),
            title: state.state.title.clone().unwrap(),
            message: state.state.user_input.clone(),
        };
        debug!("Prepared input: {:#?}", input);
        debug!("Running task...");
        let output: SectionNamesOutput = task
            .run_structured(
                API.clone(),
                &input,
                serde_json::to_string_pretty(&schema_for!(SectionNamesOutput))?,
            )
            .await?;
        debug!("Task completed");
        state.state.sections = Some(output.sections);
        debug!("Sections: {:#?}", state.state.sections);
        dbg!(&state.state.sections);
        debug!("SectionNamesJob completed");
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
    async fn test_section_job_valid() {
        env_logger::init();
        let job = SectionNamesJob;
        let state = WorkflowState {
            id: "tlksajbdfaln".into(),
            last_job_type: JobType::Pending,
            state: FullReport::new("sjaudnhcrlas".into(), "Apple stock in 2025".into())
                .with_title("State of Apple in 2025".into()),
        };
        job.run(state).await.unwrap().state.sections.unwrap();
    }
}
