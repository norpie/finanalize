use crate::{llm::API, prelude::*, prompting, tasks::Task, workflow::WorkflowState};

use async_trait::async_trait;
use models::{SubSectionsInput, SubSectionsOutput};

use super::Job;

pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SubSectionsInput {
        pub sections: Vec<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RawSubSectionsInput {
        pub input: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SubSectionsOutput {
        pub sub_sections: Vec<Vec<String>>,
    }
}

pub struct SubSectionsJob;

#[async_trait]
impl Job for SubSectionsJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let prompt = prompting::get_prompt("subsection".into())?;
        let task = Task::new(&prompt);
        let task = task.clone();
        let input = SubSectionsInput {
            sections: state.state.sections.clone().unwrap(),
        };
        let raw_input = models::RawSubSectionsInput {
            input: serde_json::to_string(&input)?,
        };
        let output: SubSectionsOutput = task.run(API.clone(), &raw_input).await?;
        state.state.sub_sections = Some(output.sub_sections);
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
    async fn test_subsection_job_valid() {
        env_logger::init();
        let job = SubSectionsJob;
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
                sections: Some(vec![
                    "Introduction".into(),
                    "Market Analysis".into(),
                    "Financial Analysis".into(),
                    "Conclusion".into(),
                ]),
                sub_sections: None,
                searches: None,
                search_results: None,
                sources: None,
                report: None,
            },
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.sub_sections.unwrap());
    }
}
