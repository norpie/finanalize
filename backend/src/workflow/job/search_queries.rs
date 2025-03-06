use async_trait::async_trait;
use chrono::Utc;
use itertools::izip;
use log::debug;
use models::{RawSearchQueriesInput, SearchQueriesInput, SearchQueriesOutput};
use schemars::schema_for;

use crate::llm::API;
use crate::tasks::Task;
use crate::workflow::job::sub_section_questions::models::{
    SectionWithQuestions, SubSectionWithQuestions,
};
use crate::{prelude::*, prompting};

use crate::workflow::WorkflowState;

use super::Job;

pub mod models {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    use crate::workflow::job::sub_section_questions::models::SectionWithQuestions;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RawSearchQueriesInput {
        pub input: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SearchQueriesInput {
        pub date: String,
        pub title: String,
        pub sections: Vec<SectionWithQuestions>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Section {
        pub section: String,
        pub sub_sections: Vec<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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
        for (section, sub_sections, sub_section_questions) in izip!(
            state.state.sections.clone().unwrap().into_iter(),
            state.state.sub_sections.clone().unwrap().into_iter(),
            state
                .state
                .sub_section_questions
                .clone()
                .unwrap()
                .into_iter(),
        ) {
            debug!("Section: {}", section);
            let mut sub_sections2 = Vec::new();
            for (sub_section, questions) in sub_sections.into_iter().zip(sub_section_questions) {
                debug!("Sub-section: {}", sub_section);
                sub_sections2.push(SubSectionWithQuestions {
                    sub_section,
                    questions,
                });
            }
            sections.push(SectionWithQuestions {
                section,
                sub_sections: sub_sections2,
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
        let output: SearchQueriesOutput = task
            .run_structured(
                API.clone(),
                &raw_input,
                serde_json::to_string_pretty(&schema_for!(SearchQueriesOutput))?,
            )
            .await?;
        debug!(
            "Generated search queries successfully. Total queries: {}",
            output.queries.len()
        );
        state.state.search_queries = Some(output.queries);
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
            state: FullReport::new("sjaudnhcrlas".into(), "Apple stock in 2025".into())
                .with_title("State of Apple in 2025".into())
                .with_sections(vec![
                    "Introduction".into(),
                    "Market Analysis".into(),
                    "Financial Analysis".into(),
                    "Conclusion".into(),
                ])
                .with_sub_sections(vec![
                    vec!["Background".into(), "Problem Statement".into()],
                    vec!["Market Size".into(), "Market Share".into()],
                    vec!["Revenue".into(), "Profit".into()],
                    vec!["Recommendation".into()],
                ])
                .with_sub_section_questions(vec![
                    vec![
                        vec![
                            "What significant events shaped Apple's position in the market up to 2025?".into(),
                        ],
                        vec![
                            "What are the main challenges Apple faces in maintaining its competitive edge by 2025?".into(),
                        ],
                    ],
                    vec![
                        vec![
                            "How has the overall market size for technology products evolved, impacting Apple's growth in 2025?".into(),
                            "What factors contribute to the projected changes in market size relevant to Apple?".into(),
                        ],
                        vec![
                            "In comparison to its competitors, what percentage of the market does Apple control in 2025?".into(),
                            "How has this market share changed from previous years, and what strategies have influenced these changes?".into(),
                        ],
                    ],
                    vec![
                        vec![
                            "What is Apple's total revenue in 2025, and how has it grown compared to previous years?".into(),
                            "Which product lines contribute the most to this revenue growth?".into(),
                        ],
                        vec![
                            "How does Apple's profit margin compare with its market share in 2025?".into(),
                            "What cost factors have influenced Apple's profitability in 2025?".into(),
                        ],
                    ],
                    vec![
                        vec![
                            "Based on the market and financial analysis, what strategic recommendations are made for Apple to sustain its growth in 2025?".into(),
                            "How do these recommendations address the challenges outlined in the problem statement?".into(),
                        ],
                    ],
                ])
                ,

        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.search_queries.unwrap());
    }
}
