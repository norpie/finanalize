use std::{env, sync::Arc};

use crate::{prelude::*, workflow::WorkflowState};

use super::Job;

use async_trait::async_trait;
use deadpool::unmanaged::{Object, Pool};
use fantoccini::{Client, ClientBuilder};
use log::debug;
use serde_json::json;
use tokio::{sync::Mutex, task::JoinSet};

pub mod models {}

pub struct ScrapePagesJob;

const BROWSER_COUNT: u16 = 4;
const FIRST_PORT: u16 = 4444;

async fn make_browsers(amount: u16) -> Result<Pool<Client>> {
    let mut browsers = vec![];
    for i in 0..amount {
        let mut default = format!("http://localhost:{}", FIRST_PORT + i);
        if let Ok(address) = env::var(format!("GECKODRIVER{}_URL", i)) {
            default = address;
        }
        browsers.push(
            ClientBuilder::native()
                .capabilities(
                    json!({
                        "moz:firefoxOptions": {
                            "args": ["--headless"]
                        }
                    })
                    .as_object()
                    .unwrap()
                    .clone(),
                )
                .connect(&default)
                .await?,
        );
    }
    Ok(Pool::from(browsers))
}

async fn scrape_page(browser: &Object<Client>, url: &str) -> Result<String> {
    browser.goto(url).await?;
    let source = browser.source().await?;
    Ok(source)
}

fn split_evenly(items: Vec<String>, n: usize) -> Vec<Vec<String>> {
    let mut chunks = vec![Vec::new(); n];
    for (index, item) in items.into_iter().enumerate() {
        chunks[index % n].push(item);
    }
    chunks
}

#[async_trait]
impl Job for ScrapePagesJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let browsers = Arc::new(make_browsers(BROWSER_COUNT).await?);
        let sources: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
        let search_results = state.state.search_results.clone().unwrap();
        let mut join_set = JoinSet::new();
        let total = search_results.len();
        for (i, source) in search_results.into_iter().enumerate() {
            let browsers = browsers.clone();
            let sources = sources.clone();
            join_set.spawn(async move {
                let Ok(browser) = browsers.clone().get().await else {
                    return Err(FinanalizeError::InternalServerError);
                };
                debug!("Scraping ({}/{}): {}", i + 1, total, source);
                let Ok(source) = scrape_page(&browser, &source).await else {
                    return Err(FinanalizeError::InternalServerError);
                };
                sources.clone().lock().await.push(source);
                Ok(())
            });
        }

        let results: Result<Vec<()>> = join_set.join_all().await.into_iter().collect();
        results?;

        for _ in 0..BROWSER_COUNT {
            let browser = browsers.remove().await?;
            browser.close().await?;
        }
        state.state.sources = Some(sources.lock().await.clone());
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
        let job = ScrapePagesJob;
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
                search_results: Some(
                    vec![
                        "https://backlinko.com/apple-statistics",
                        "https://blog.tbrc.info/2025/02/apples-market-demand/",
                        "https://capital.com/en-eu/analysis/apple-stock-price-in-10-years",
                        "https://coincodex.com/stock/AAPL/price-prediction/",
                        "https://cyble.com/blog/apple-fixes-cve-2025-24085-security-update/",
                        "https://www.businessofapps.com/data/apple-statistics/",
                        "https://www.captide.co/insights/apple-q1-2025",
                        "https://www.cnbc.com/2025/01/30/apple-aapl-q1-earnings-2025.html",
                        "https://www.cultofmac.com/apple-history/apple-incorporation",
                        "https://www.nasdaq.com/articles/history-apple-company-and-stock",
                        "https://www.nasdaq.com/articles/what-lies-ahead-apple-stock-etfs-2025",
                        "https://www.officetimeline.com/blog/apple-inc-timeline",
                        "https://www.technavio.com/report/fresh-apples-market-industry-analysis",
                    ]
                    .into_iter()
                    .map(Into::into)
                    .collect(),
                ),
                sources: None,
                report: None,
            },
        };
        let _state = job.run(state).await.unwrap();
        // dbg!(state.state.sources.unwrap());
    }
}
