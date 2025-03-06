use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use log::debug;
use models::FormatContentJobInput;
use tokio::{sync::Semaphore, task::JoinHandle};

use crate::{
    llm::API, models::PreClassificationSource, prelude::*, prompting, tasks::Task,
    workflow::WorkflowState,
};

use super::Job;

pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FormatContentJobInput {
        pub date: String,
        pub content: String,
        pub url: String,
    }
}

pub struct FormatContentJob;

#[async_trait]
impl Job for FormatContentJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let prompt = prompting::get_prompt("source-formatter".into())?;
        let task = Task::new(&prompt);
        let md_sources = state.state.md_sources.clone().unwrap();
        let max_jobs = 1;
        let sem = Arc::new(Semaphore::new(max_jobs));
        let len = md_sources.len();
        let mut handles = vec![];
        for (i, source) in md_sources.into_iter().enumerate() {
            let task = task.clone();
            let sem = sem.clone();
            let handle: JoinHandle<Result<PreClassificationSource>> = tokio::spawn(async move {
                let permit = sem.acquire().await.unwrap();
                debug!("Formatting source {} of {}", i + 1, len);
                let input = FormatContentJobInput {
                    date: Utc::now().format("%Y-%m-%d").to_string(),
                    content: source.content,
                    url: source.url,
                };
                let output = task.run_raw(API.clone(), &input).await?;
                let source = PreClassificationSource {
                    url: input.url,
                    content: output,
                };
                drop(permit);
                Ok(source)
            });
            handles.push(handle);
        }
        let mut sources = Vec::new();
        for handle in handles {
            let source = handle.await??;
            sources.push(source);
        }
        // sources.push(source);
        state.state.md_sources = Some(sources);
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        models::{FullReport, PreClassificationSource},
        workflow::JobType,
    };

    #[tokio::test]
    #[ignore = "uses llm api (external service)"]
    async fn test_classify_job_valid() {
        env_logger::init();
        let job = FormatContentJob;
        let state = WorkflowState {
            id: "tlksajbdfaln".into(),
            last_job_type: JobType::Pending,
            state: FullReport::new("sjaudnhcrlas".into(), "apple stock in 2025".into())
                .with_title("state of apple in 2025".into())
                .with_sections(vec![
                    "introduction".into(),
                    "market analysis".into(),
                    "financial analysis".into(),
                    "conclusion".into(),
                ])
                .with_sub_sections(vec![
                    vec!["background".into(), "problem statement".into()],
                    vec!["market size".into(), "market share".into()],
                    vec!["revenue".into(), "profit".into()],
                    vec!["recommendation".into()],
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
                    "https://coincodex.com/stock/aapl/price-prediction/".into(),
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
                    content: include_str!("../../../tests/scraped/nbc.md").into(),
                }]),
        };
        let state = job.run(state).await.unwrap();
        println!(
            "{}",
            state.state.md_sources.unwrap().first().unwrap().content
        );
    }
}
