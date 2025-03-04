use async_trait::async_trait;
use itertools::izip;
use models::{AnswerQuestionsInput, QuestionAnswer};

use crate::llm::API;
use crate::rag::DistancedChunk;
use crate::tasks::Task;
use crate::{prelude::*, prompting, rag};

use crate::workflow::WorkflowState;

use super::index_chunks::models::EmbeddedChunk;
use super::{sub_sections, Job};

pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct AnswerQuestionsInput {
        pub context: String,
        pub title: String,
        pub section: String,
        #[serde(rename = "subSection")]
        pub sub_section: String,
        pub question: String,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct AnswerQuestionsOutput {
        pub answer: String,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct QuestionAnswer {
        pub question: String,
        pub answer: String,
    }
}

pub struct AnswerQuestionsJob;

pub struct W(Vec<DistancedChunk>);

impl W {
    fn into_context(self, len: usize) -> String {
        let mut context = String::new();
        for chunk in self.0 {
            context.push_str("***");
            context.push_str(&format!("# START - Source ID: {}", chunk.source_id));
            context.push_str("***");
            context.push_str(&chunk.chunk);
            context.push_str(&format!("# STOP - Source ID: {}", chunk.source_id));
            if context.len() >= len {
                break;
            }
        }
        context
    }
}

#[async_trait]
impl Job for AnswerQuestionsJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let prompt = prompting::get_prompt("answer-questions".into())?;
        let task = Task::new(&prompt);
        let mut pairs = Vec::new();
        for (section_name, sub_sections, sub_section_questions) in izip!(
            state.state.sections.clone().unwrap().into_iter(),
            state.state.sub_sections.clone().unwrap().into_iter(),
            state
                .state
                .sub_section_questions
                .clone()
                .unwrap()
                .into_iter(),
        ) {
            let mut section = Vec::new();
            for (sub_section_name, questions) in sub_sections.into_iter().zip(sub_section_questions)
            {
                let mut sub_section = Vec::new();
                for question in questions.into_iter() {
                    let context = rag::vector_search(
                        ("report", state.id.as_str()).into(),
                        question.to_string(),
                    )
                    .await?;
                    if context.is_empty() {
                        panic!(
                            "Empty context for report:{}({}) and question: {}",
                            state.id.clone(),
                            state.state.id.clone(),
                            question
                        );
                    }
                    let input = AnswerQuestionsInput {
                        context: W(context).into_context(4096),
                        title: state.state.title.clone().unwrap(),
                        section: section_name.clone(),
                        sub_section: sub_section_name.clone(),
                        question: question.clone(),
                    };
                    let answer = task.run_raw(API.clone(), &input).await?;
                    sub_section.push(QuestionAnswer { question, answer });
                }
                section.push(sub_section);
            }
            pairs.push(section);
        }
        state.state.question_answer_pairs = Some(pairs);
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    use crate::{
        db::{self, DB},
        models::FullReport,
        workflow::{job::classify_sources::models::ClassifySourcesOutput, JobType, WorkflowState},
    };

    #[tokio::test]
    #[ignore = "Uses LLM API (External Service)"]
    async fn test_classify_job_valid() {
        env_logger::init();
        DB.set(db::connect().await.unwrap()).unwrap();
        let job = AnswerQuestionsJob;
        let state = WorkflowState {
            id: "sjaudnhcrlas".into(),
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
                .with_sources(vec![
                    ClassifySourcesOutput {
                        title: "Apple shares rise 3% as boost in services revenue overshadows iPhone miss".into(),
                        summary: "Apple’s overall revenue rose 4% in its first fiscal quarter, but it missed on Wall Street’s iPhone sales expectations and saw sales in China decline 11.1%, the company reported Thursday.".into(),
                        author: "Anonymous".into(),
                        published_after: Some(Utc::now().to_rfc3339()),
                        date: Some(Utc::now().to_rfc3339())
                    }
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
        dbg!(state.state.question_answer_pairs.unwrap());
    }
}
