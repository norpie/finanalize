use async_trait::async_trait;
use itertools::izip;
use log::debug;
use models::{AnswerQuestionsInput, QuestionAnswer};

use crate::llm::API;
use crate::rag::DistancedChunk;
use crate::tasks::Task;
use crate::{prelude::*, prompting, rag};

use crate::workflow::WorkflowState;

use super::Job;

pub mod models {
    use serde::{Deserialize, Serialize};

    use crate::rag::DistancedChunk;

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct AnswerQuestionsInput {
        pub sources: Vec<DistancedChunk>,
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
    // Only keep top results which make string length less than len
    fn into_context(self, len: usize) -> Vec<DistancedChunk> {
        let mut total_len = 0;
        let mut res = Vec::new();
        for chunk in self.0.into_iter() {
            if total_len + chunk.chunk.len() > len {
                break;
            }
            total_len += chunk.chunk.len();
            res.push(chunk);
        }
        res
    }
}

#[async_trait]
impl Job for AnswerQuestionsJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        debug!("Running AnswerQuestionsJob for report {}", state.id);
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
                        sources: W(context).into_context(8192),
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
        workflow::{job::classify_sources::models::ClassifiedSource, JobType, WorkflowState},
    };

    #[tokio::test]
    #[ignore = "Uses LLM API (External Service)"]
    async fn test_classify_job_valid() {
        env_logger::init();
        DB.set(db::connect().await.unwrap()).unwrap();
        let job = AnswerQuestionsJob;
        let state = WorkflowState {
            id: "asdlfjhasldfjh".into(),
            last_job_type: JobType::Pending,
            state: FullReport::new("asdlfjhasldfjh".into(), "Apple stock in 2025".into())
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
                    ClassifiedSource {
                        id: "0".into(),
                        content: r#"# Apple shares rise 3% as boost in services revenue overshadows iPhone miss

> Kif Leswing, CNBC
> Published on January 30, 2025

Apple's overall revenue rose 4% during its first fiscal quarter, but it missed on Wall Street's iPhone sales expectations and saw sales in China decline 11.1%, the company reported Thursday.

Although Apple's overall sales rose during the quarter, the company's closely watched iPhone sales declined slightly on a year-over-year basis. The December quarter is the first full quarter with iPhone 16 sales, and Apple released its Apple Intelligence AI suite for the devices during the quarter.

Apple's profit engine, its Services division, which includes subscriptions, warranties and licensing deals, reported $23.12 billion in revenue, which is 14% higher than the same period last year. Apple CEO Tim Cook told analysts on a call Thursday that the company had more than one billion subscriptions, which includes both direct subscriptions for services such as Apple TV+ and iCloud, as well as subscriptions to third-party apps through the company's App Store system.

The December quarter is the first full quarter with iPhone 16 sales, and Apple released its Apple Intelligence AI suite for the devices during the quarter.

Apple said it expected growth in the March quarter of "low to mid single digits" on an annual basis. The company also said it expected "low double digits" growth for its Services division.

## Secondary numbers
- $2.40 - Earnings per share
- $124.30 billion - Revenue
- $69.14 billion - iPhone revenue
- $8.99 billion - Mac revenue
- $8.09 billion - iPad revenue
- $11.75 billion - Other products revenue
- $26.34 billion - Services revenue
- 46.9% - Gross margin"#.into(),
                        url: "https://www.nbcboston.com/news/business/money-report/apple-reports-first-quarter-earnings-after-the-bell-2/3617779/?os=android&ref=app&noamp=mobile".into(),
                        title: "Apple shares rise 3% as boost in services revenue overshadows iPhone miss".into(),
                        author: "Kif Leswing, CNBC".into(),
                        published_after: Some(Utc::now().format("%Y-%m-%d").to_string()),
                        date: Some(Utc::now().format("%Y-%m-%d").to_string()),
                    }
                ])
                .with_sub_section_questions(vec![
                    vec![
                        vec![
                            "What did Apple's Q1 2025 look like?".into(),
                        ],
                    ],
                ])
                ,
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.question_answer_pairs.unwrap());
    }
}
