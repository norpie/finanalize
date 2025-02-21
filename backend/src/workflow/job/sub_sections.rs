use crate::{llm::API, prelude::*, prompting, tasks::Task, workflow::WorkflowState};

use async_trait::async_trait;
use models::{SubSectionsInput, SubSectionsOutput};
use tokio::task::JoinHandle;

use super::Job;

pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SubSectionsInput {
        pub message: String,
        pub title: String,
        pub section: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SubSectionsOutput {
        pub sub_sections: Vec<String>,
    }
}

pub struct SubSectionsJob;

#[async_trait]
impl Job for SubSectionsJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let prompt = prompting::get_prompt("subsection".into())?;
        let task = Task::new(&prompt);
        let mut handles = vec![];
        for (i, section) in state
            .state
            .sections
            .clone()
            .unwrap()
            .into_iter()
            .enumerate()
        {
            let state = state.state.clone();
            let task = task.clone();
            let handle: JoinHandle<Result<Vec<String>>> = tokio::spawn(async move {
                let input = SubSectionsInput {
                    message: state.user_input,
                    title: state.title.unwrap(),
                    section,
                };
                let output: SubSectionsOutput = task.run(API.clone(), &input).await?;
                Ok(output.sub_sections)
            });
            handles.push((i, handle));
        }
        let mut indexed_sub_sections = Vec::with_capacity(handles.len());
        for (i, handle) in handles {
            let result = handle.await?;
            indexed_sub_sections.push((i, result?));
        }
        indexed_sub_sections.sort_by_key(|(i, _)| *i);
        let sub_sections = indexed_sub_sections
            .into_iter()
            .map(|(_, sub_sections)| sub_sections)
            .collect();
        state.state.sub_sections = Some(sub_sections);
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
            },
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.sub_sections.unwrap());
    }
}
