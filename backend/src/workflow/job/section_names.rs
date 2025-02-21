use crate::{llm::API, prelude::*, prompting, tasks::Task};

use async_trait::async_trait;
use models::{SectionNamesInput, SectionNamesOutput};

use crate::workflow::WorkflowState;

use super::Job;

pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct SectionNamesInput {
        pub title: String,
        pub message: String,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct SectionNamesOutput {
        pub sections: Vec<String>,
    }
}

pub struct SectionNamesJob;

#[async_trait]
impl Job for SectionNamesJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let prompt = prompting::get_prompt("section".into())?;
        let task = Task::new(&prompt);
        let input = SectionNamesInput {
            title: state.state.title.clone().unwrap(),
            message: state.state.user_input.clone(),
        };
        let output: SectionNamesOutput = task.run(API.clone(), &input).await?;
        state.state.sections = Some(output.sections);
        dbg!(&state.state.sections);
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
            state: FullReport {
                id: "sjaudnhcrlas".into(),
                user_input: "Apple stock in 2025".into(),
                status: JobType::Pending,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                validation: None,
                title: Some("State of Apple in 2025".into()),
                sections: None,
                sub_sections: None,
                searches: None,
                search_results: None,
                sources: None,
            },
        };
        job.run(state).await.unwrap().state.sections.unwrap();
    }
}
