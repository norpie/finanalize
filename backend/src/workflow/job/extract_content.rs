use async_trait::async_trait;
use log::debug;

use crate::prelude::*;

use crate::workflow::WorkflowState;

use super::Job;

pub struct ExtractContentJob;

#[async_trait]
impl Job for ExtractContentJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        // let extractor = HTMLExtractor;
        let mut mds = vec![];
        let html_sources = state.state.html_sources.clone().unwrap();
        let total = html_sources.len();
        for (i, html) in html_sources.into_iter().enumerate() {
            debug!("Extracting content from HTML source ({}/{})", i + 1, total);
            // let content = extractor
            //     .extract(FileType::Html(html))
            //     .await?
            //     .into_iter()
            //     .next()
            //     .ok_or(FinanalizeError::ParseError("No content extracted".into()))?;
            // match content {
            //     Content::MarkDown(md) => mds.push(md),
            //     _ => continue,
            // }
            mds.push(mdka::from_html(&html))
        }
        state.state.raw_sources = Some(mds);
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
        let job = ExtractContentJob;
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
                .with_html_sources(vec![
                    include_str!("../../../tests/scraped/capital.html").into(),
                    include_str!("../../../tests/scraped/captide.html").into(),
                    include_str!("../../../tests/scraped/tbrc.html").into(),
                ]),
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.raw_sources.unwrap());
    }
}
