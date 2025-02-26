use async_trait::async_trait;
use chrono::Utc;
use log::debug;
use models::{RawSearchQueriesInput, SearchQueriesInput, SearchQueriesOutput, Section};

use crate::llm::API;
use crate::tasks::Task;
use crate::{prelude::*, prompting};

use crate::workflow::WorkflowState;

use super::Job;

pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RawSearchQueriesInput {
        pub input: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SearchQueriesInput {
        pub date: String,
        pub title: String,
        pub sections: Vec<Section>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Section {
        pub section: String,
        pub sub_sections: Vec<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SearchQueriesOutput {
        pub queries: Vec<String>,
    }
}

pub struct GenerateSearchQueriesJob;

#[async_trait]
impl Job for GenerateSearchQueriesJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        debug!("Running GenerateSearchQueriesJob...");
        let prompt = prompting::get_prompt("search".into())?;
        let task = Task::new(&prompt);
        let mut sections = Vec::new();
        for (section, sub_sections) in state
            .state
            .sections
            .clone()
            .unwrap()
            .into_iter()
            .zip(state.state.sub_sections.clone().unwrap())
        {
            debug!("Section: {}", section);
            debug!("Sub-sections: {:?}", sub_sections);
            sections.push(Section {
                section,
                sub_sections,
            });
        }
        let input = SearchQueriesInput {
            title: state.state.title.clone().unwrap(),
            date: Utc::now().to_rfc3339(),
            sections,
        };
        debug!(
            "Serialized input for search queries: {}",
            serde_json::to_string_pretty(&input)?
        );
        let raw_input = RawSearchQueriesInput {
            input: serde_json::to_string_pretty(&input)?,
        };
        debug!("Running task to generate search queries...");
        let output: SearchQueriesOutput = task.run(API.clone(), &raw_input).await?;
        debug!(
            "Generated search queries successfully. Total queries: {}",
            output.queries.len()
        );
        state.state.searches = Some(output.queries);
        debug!("GenerateSearchQueriesJob completed");
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
    async fn test_searches_job_valid() {
        env_logger::init();
        let job = GenerateSearchQueriesJob;
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
                    "Introduction to Apple".into(),
                    "Market Analysis and Forecast".into(),
                    "Financial Analysis of Apple".into(),
                    "Conclusion".into(),
                ]),
                sub_sections: Some(vec![
                    vec!["Background".into(), "Problem Statement".into()],
                    vec!["Market Size".into(), "Market Share".into()],
                    vec!["Revenue".into(), "Profit".into()],
                    vec!["Recommendation".into()],
                ]),
                searches: None,
                search_results: None,
                sources: None,
                report: None,
            },
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.searches.unwrap());
    }
}
