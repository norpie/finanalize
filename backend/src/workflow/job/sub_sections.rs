use crate::{llm::API, prelude::*, prompting, tasks::Task, workflow::WorkflowState};

use async_trait::async_trait;
use log::debug;
use models::{SubSectionsInput, SubSectionsOutput};
use schemars::schema_for;

use super::Job;

pub mod models {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SubSectionsInput {
        pub title: String,
        pub message: String,
        pub sections: Vec<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RawSubSectionsInput {
        pub input: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    pub struct SubSectionsOutput {
        pub sub_sections: Vec<Vec<String>>,
    }
}

pub struct SubSectionsJob;

#[async_trait]
impl Job for SubSectionsJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        debug!("Running SubSectionsJob...");
        let prompt = prompting::get_prompt("subsection".into())?;
        let task = Task::new(&prompt);
        let task = task.clone();
        let input = SubSectionsInput {
            message: state.state.user_input.clone(),
            title: state.state.title.clone().unwrap(),
            sections: state.state.sections.clone().unwrap(),
        };
        debug!("Prepared input: {:#?}", input);
        let raw_input = models::RawSubSectionsInput {
            input: serde_json::to_string(&input)?,
        };
        debug!("Serialized input for task: {:#?}", raw_input.input);
        debug!("Running task...");
        let output: SubSectionsOutput = task
            .run_structured(
                API.clone(),
                &raw_input,
                serde_json::to_string_pretty(&schema_for!(SubSectionsOutput))?,
            )
            .await?;
        debug!("Task completed");
        state.state.sub_sections = Some(output.sub_sections);
        debug!("Sub-sections: {:#?}", state.state.sub_sections);
        dbg!(&state.state.sub_sections);
        debug!("SubSectionsJob completed");
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
            state: FullReport::new("sjaudnhcrlas".into(), "Apple stock in 2025".into())
                .with_title("State of Apple in 2025".into())
                .with_sections(vec![
                    "Introduction".into(),
                    "Market Analysis".into(),
                    "Financial Analysis".into(),
                    "Conclusion".into(),
                ]),
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.sub_sections.unwrap());
    }
}
