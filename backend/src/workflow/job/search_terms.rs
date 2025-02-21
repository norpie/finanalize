use crate::{prelude::*, search::SEARCH, workflow::WorkflowState};

use async_trait::async_trait;

use super::Job;

pub struct SearchJob;

#[async_trait]
impl Job for SearchJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let mut all_urls = vec![];
        for search in state.state.searches.clone().unwrap() {
            let result = SEARCH.clone().search(&search).await?;
            all_urls.extend(result);
        }
        all_urls.sort();
        all_urls.dedup();
        state.state.sources = Some(all_urls);
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
                searches: Some(
                    vec![
                        "background on apple company 2025",
                        "history of apple corporation 2025",
                        "origins of apple technology 2025",
                        "apple problem statement 2025",
                        "challenges faced by apple in 2025",
                        "issues affecting apple business in 2025",
                        "apple market size forecast 2025",
                        "growth projection for apple market 2025",
                        "expected apple market value 2025",
                        "apple market share analysis 2025",
                        "market position of apple in 2025",
                        "apple's share in global tech market 2025",
                        "revenue trends for apple 2025",
                        "apple financial performance revenue 2025",
                        "annual revenue forecast for apple 2025",
                        "profit analysis of apple 2025",
                        "net profit forecast for apple 2025",
                        "apple's profitability in 2025",
                    ]
                    .into_iter()
                    .map(Into::into)
                    .collect(),
                ),
                sources: None,
            },
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.sources.unwrap());
    }
}
