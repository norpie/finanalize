use async_trait::async_trait;
use log::debug;
use models::SectionizeQuestionsJobInput;

use crate::llm::API;
use crate::tasks::Task;
use crate::{prelude::*, prompting};

use crate::workflow::WorkflowState;

use super::Job;

pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct SectionizeQuestionsJobInput {
        pub input: String,
    }
}

pub struct SectionizeQuestionsJob;

#[async_trait]
impl Job for SectionizeQuestionsJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let prompt = prompting::get_prompt("sectionize-questions".into())?;
        let task = Task::new(&prompt);
        let mut sections = Vec::new();
        let sections_vec = state.state.question_answer_pairs.clone().unwrap();
        let sections_len = sections_vec.len();
        for (i, section_vec) in sections_vec.iter().enumerate() {
            let sub_sections_len = section_vec.len();
            let mut sub_sections = Vec::new();
            for (j, sub_section_vec) in section_vec.iter().enumerate() {
                let mut content = String::new();
                for qa_pair in sub_section_vec {
                    content.push('#');
                    content.push_str(&qa_pair.question);
                    content.push('\n');
                    content.push('\n');
                    content.push_str(&qa_pair.answer);
                    content.push('\n');
                }
                debug!(
                    "Running task for section {} of {} and sub-section {} of {}",
                    i + 1,
                    sections_len,
                    j + 1,
                    sub_sections_len
                );
                let res = task
                    .run_raw(API.clone(), &SectionizeQuestionsJobInput { input: content })
                    .await?;
                let sub_section_content = res.output;
                state.state.generation_results.push(res.info);
                sub_sections.push(sub_section_content);
            }
            sections.push(sub_sections);
        }
        state.state.sub_section_contents = Some(sections);
        Ok(state)
    }
}
