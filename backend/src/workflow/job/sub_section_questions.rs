use crate::{
    llm::API, prelude::*, prompting, tasks::Task, workflow::job::search_queries::models::Section,
};

use async_trait::async_trait;
use chrono::Utc;
use log::debug;
use models::{RawSubSectionQuestionsInput, SubSectionQuestionsInput, SubSectionQuestionsOutput};
use schemars::schema_for;

use crate::workflow::WorkflowState;

use super::Job;

pub mod models {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    use crate::workflow::job::search_queries::models::Section;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RawSubSectionQuestionsInput {
        pub input: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SubSectionQuestionsInput {
        pub title: String,
        pub date: String,
        pub sections: Vec<Section>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    pub struct SubSectionQuestionsOutput {
        pub sections: Vec<SectionWithQuestions>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    pub struct SectionWithQuestions {
        pub section: String,
        pub sub_sections: Vec<SubSectionWithQuestions>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    pub struct SubSectionWithQuestions {
        pub sub_section: String,
        pub questions: Vec<String>,
    }
}

pub struct SubSectionQuestionsJob;

#[async_trait]
impl Job for SubSectionQuestionsJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
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
        let input = SubSectionQuestionsInput {
            title: state.state.title.clone().unwrap(),
            date: Utc::now().to_rfc3339(),
            sections,
        };
        let raw_input = RawSubSectionQuestionsInput {
            input: serde_json::to_string_pretty(&input)?,
        };
        println!("input: {}", &raw_input.input);
        let prompt = prompting::get_prompt("sub-section-questions".into())?;
        let task = Task::new(&prompt);
        let output: SubSectionQuestionsOutput = task
            .run_structured(
                API.clone(),
                &raw_input,
                serde_json::to_string_pretty(&schema_for!(SubSectionQuestionsOutput))?,
            )
            .await?;
        let mut sections: Vec<Vec<Vec<String>>> = Vec::new();
        for section in output.sections {
            let mut sub_sections: Vec<Vec<String>> = Vec::new();
            debug!("Section: {}", section.section);
            for sub_section in section.sub_sections {
                sub_sections.push(sub_section.questions);
            }
            sections.push(sub_sections);
        }
        state.state.sub_section_questions = Some(sections);
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
        let job = SubSectionQuestionsJob;
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
                ]),
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.sub_section_questions.unwrap());
    }
}
