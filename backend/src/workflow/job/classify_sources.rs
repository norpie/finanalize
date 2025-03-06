use async_trait::async_trait;
use models::{ClassifiedSource, ClassifySourcesInput, ClassifySourcesOutput};
use schemars::schema_for;

use crate::{llm::API, prelude::*, prompting, tasks::Task, workflow::WorkflowState};

use super::Job;

pub mod models {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    use crate::models::PreClassificationSource;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ClassifySourcesInput {
        pub input: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    pub struct ClassifySourcesOutput {
        pub title: String,
        pub author: String,
        pub date: Option<String>,
        #[serde(rename = "publishedAfter")]
        pub published_after: Option<String>,
    }

    impl ClassifiedSource {
        pub fn from_id(id: String, value: ClassifySourcesOutput, pre: PreClassificationSource) -> Self {
            Self {
                id,
                title: value.title,
                author: value.author,
                date: value.date,
                published_after: value.published_after,
                url: pre.url,
                content: pre.content,
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ClassifiedSource {
        pub id: String,
        pub title: String,
        pub author: String,
        pub date: Option<String>,
        #[serde(rename = "publishedAfter")]
        pub published_after: Option<String>,
        pub url: String,
        pub content: String,
    }
}

pub struct ClassifySourcesJob;

#[async_trait]
impl Job for ClassifySourcesJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let prompt = prompting::get_prompt("content-classifier".into())?;
        let task = Task::new(&prompt);
        let mut sources = Vec::new();
        for (i, source) in state
            .state
            .md_sources
            .clone()
            .unwrap()
            .into_iter()
            .enumerate()
        {
            let input = ClassifySourcesInput {
                input: source.content.clone(),
            };
            let output: ClassifySourcesOutput = task
                .run_structured(
                    API.clone(),
                    &input,
                    serde_json::to_string_pretty(&schema_for!(ClassifySourcesOutput))?,
                )
                .await?;
            sources.push(ClassifiedSource::from_id(format!("website{}", i), output, source));
        }
        state.state.sources = Some(sources);
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        models::{FullReport, PreClassificationSource},
        workflow::{JobType, WorkflowState},
    };

    #[tokio::test]
    #[ignore = "Uses LLM API (External Service)"]
    async fn test_classify_job_valid() {
        env_logger::init();
        let job = ClassifySourcesJob;
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
                .with_searches(vec![
                    "background on apple company 2025".into(),
                    "history of apple corporation 2025".into(),
                    "origins of apple technology 2025".into(),
                    "apple problem statement 2025".into(),
                    "challenges faced by apple in 2025".into(),
                    "issues affecting apple business in 2025".into(),
                    "apple market size forecast 2025".into(),
                    "growth projection for apple market 2025".into(),
                    "expected apple market value 2025".into(),
                    "apple market share analysis 2025".into(),
                    "market position of apple in 2025".into(),
                    "apple's share in global tech market 2025".into(),
                    "revenue trends for apple 2025".into(),
                    "apple financial performance revenue 2025".into(),
                    "annual revenue forecast for apple 2025".into(),
                    "profit analysis of apple 2025".into(),
                    "net profit forecast for apple 2025".into(),
                    "apple's profitability in 2025".into(),
                ])
                .with_search_results(vec![
                    "https://backlinko.com/apple-statistics".into(),
                    "https://blog.tbrc.info/2025/02/apples-market-demand/".into(),
                    "https://capital.com/en-eu/analysis/apple-stock-price-in-10-years".into(),
                    "https://coincodex.com/stock/AAPL/price-prediction/".into(),
                    "https://cyble.com/blog/apple-fixes-cve-2025-24085-security-update/".into(),
                    "https://www.businessofapps.com/data/apple-statistics/".into(),
                    "https://www.captide.co/insights/apple-q1-2025".into(),
                    "https://www.cnbc.com/2025/01/30/apple-aapl-q1-earnings-2025.html".into(),
                    "https://www.cultofmac.com/apple-history/apple-incorporation".into(),
                    "https://www.nasdaq.com/articles/history-apple-company-and-stock".into(),
                    "https://www.nasdaq.com/articles/what-lies-ahead-apple-stock-etfs-2025".into(),
                    "https://www.officetimeline.com/blog/apple-inc-timeline".into(),
                    "https://www.technavio.com/report/fresh-apples-market-industry-analysis".into(),
                ])
                .with_raw_sources(vec![PreClassificationSource {
                    url: "https://www.nbcboston.com/news/business/money-report/apple-reports-first-quarter-earnings-after-the-bell-2/3617779/?os=android&ref=app&noamp=mobile".into(),
                    content: include_str!("../../../tests/scraped/nbc-summary.md").into(),
                }]),
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.sources.unwrap());
    }
}
