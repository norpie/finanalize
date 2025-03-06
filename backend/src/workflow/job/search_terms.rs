use crate::{prelude::*, search::SEARCH, workflow::WorkflowState};

use async_trait::async_trait;
use log::debug;
use tokio::task::JoinSet;

use super::Job;

pub struct SearchJob;

#[async_trait]
impl Job for SearchJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        debug!("Running SearchJob...");
        let searches = state.state.search_queries.clone().unwrap();
        let total = searches.len();
        debug!("Total searches: {}", total);
        // Create a vector to hold the futures
        let mut search_futures = JoinSet::new();

        for (i, search) in searches.into_iter().enumerate() {
            debug!("Searching for {} ({}/{})", search, i + 1, total);
            // Spawn each search as a separate task and push the future to the vector
            search_futures.spawn(async move { SEARCH.clone().search(&search).await });
        }

        // Join all futures concurrently
        let search_results = search_futures.join_all().await;
        let mut all_urls = Vec::new();
        for result in search_results.into_iter() {
            debug!("Adding search result: {:?}", result);
            all_urls.extend(result?);
        }

        // Sort and deduplicate URLs
        debug!("Sorting and deduplicating URLs");
        all_urls.sort();
        all_urls.dedup();
        debug!("Search results: {:?}", all_urls.len());
        state.state.search_urls = Some(all_urls);
        debug!("SearchJob completed");
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
    async fn test_search_job_valid() {
        env_logger::init();
        let job = SearchJob;
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
                ]),
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.search_urls.unwrap());
    }
}
