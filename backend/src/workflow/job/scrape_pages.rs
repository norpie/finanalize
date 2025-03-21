use std::{sync::Arc, time::Duration};

use crate::{models::PreClassificationSource, prelude::*, workflow::WorkflowState};

use super::Job;

use async_trait::async_trait;
use deadpool::unmanaged::{Object, Pool};
use fantoccini::{Client, ClientBuilder};
use log::debug;
use serde_json::json;
use tokio::{sync::Mutex, task::JoinSet, time::timeout};

pub mod models {}

pub struct ScrapePagesJob;

const BROWSER_COUNT: u16 = 4;
const FIRST_PORT: u16 = 4444;

pub async fn make_browsers(amount: u16) -> Result<Pool<Client>> {
    debug!("Initializing {} browsers...", amount);
    let mut browsers = vec![];
    for i in 0..amount {
        debug!("Initializing browser {}...", i + 1);
        let default = format!("http://localhost:{}", FIRST_PORT + i);
        // if let Ok(address) = env::var(format!("GECKODRIVER{}_URL", i)) {
        // debug!("Using custom address for browser {}: {}", i + 1, address);
        // default = address;
        // }
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
        debug!("Initialized browser {}", i + 1);
    }
    Ok(Pool::from(browsers))
}

pub async fn scrape_page(browser: &Object<Client>, url: &str) -> Result<String> {
    // browser.goto(url).await?;
    let goto_result = timeout(Duration::from_secs(2), browser.goto(url)).await;
    match goto_result {
        Ok(Ok(_)) => {
            let source = browser.source().await?;
            Ok(source)
        }
        Ok(Err(e)) => Err(e.into()), // Handle `goto` errors
        Err(_) => Err(FinanalizeError::ScraperTimemout(url.into())),
    }
    // let source = browser.source().await?;
    // Ok(source)
}

pub fn split_evenly(items: Vec<String>, n: usize) -> Vec<Vec<String>> {
    let mut chunks = vec![Vec::new(); n];
    for (index, item) in items.into_iter().enumerate() {
        chunks[index % n].push(item);
    }
    chunks
}

#[async_trait]
impl Job for ScrapePagesJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        debug!("Running ScrapePagesJob...");
        let browsers = Arc::new(make_browsers(BROWSER_COUNT).await?);
        debug!("Initialized {} browsers", BROWSER_COUNT);
        let sources: Arc<Mutex<Vec<PreClassificationSource>>> = Arc::new(Mutex::new(vec![]));
        let search_results = state.state.search_urls.clone().unwrap();
        debug!("Pages to scrape: {}", search_results.len());
        let mut join_set = JoinSet::new();
        let total = search_results.len();
        for (i, source) in search_results.into_iter().enumerate() {
            let browsers = browsers.clone();
            let sources = sources.clone();
            debug!("Spawning task for scraping URL {}: {}", i + 1, source);
            join_set.spawn(async move {
                let Ok(browser) = browsers.clone().get().await else {
                    return Err(FinanalizeError::InternalServerError);
                };
                debug!("Scraping ({}/{}): {}", i + 1, total, source);
                let Ok(html) = scrape_page(&browser, &source).await else {
                    debug!("Failed to scrape page: {}", source);
                    // return Err(FinanalizeError::InternalServerError);
                    return Ok(());
                };
                debug!("Scraped ({}/{}): {}", i + 1, total, &source);
                sources.clone().lock().await.push(PreClassificationSource {
                    url: source,
                    content: html,
                });
                Ok(())
            });
        }

        let results: Result<Vec<()>> = join_set.join_all().await.into_iter().collect();
        results?;
        debug!("Scraped all pages, closing browser instances...");

        for _ in 0..BROWSER_COUNT {
            let browser = browsers.remove().await?;
            browser.close().await?;
        }
        debug!("Closed all browser instances");
        state.state.html_sources = Some(sources.lock().await.clone());
        debug!("ScrapePagesJob completed");
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
                ]),
        };
        let _state = job.run(state).await.unwrap();
        // dbg!(state.state.sources.unwrap());
    }
}
