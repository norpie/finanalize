use async_trait::async_trait;

use crate::latex::{self, LatexComponent, Section, Source, Subsection};
use crate::prelude::*;

use crate::workflow::WorkflowState;

use super::Job;

pub mod models {}

pub struct GenerateReportJob;

#[async_trait]
impl Job for GenerateReportJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let mut sources = vec![];
        for (i, source) in state
            .state
            .search_results
            .clone()
            .unwrap()
            .into_iter()
            .enumerate()
        {
            sources.push(Source::new(
                "Website".into(),
                i.to_string(),
                "Author".into(),
                "Title".into(),
                2025,
                "Journal".into(),
                source,
            ));
        }

        let mut components = Vec::new();
        for (section, sub_sections) in state.state.sections.clone().unwrap().into_iter().zip(
            state
                .state
                .sub_sections
                .clone()
                .unwrap()
                .into_iter()
                .map(|sub_sections| sub_sections.into_iter()),
        ) {
            components.push(LatexComponent::Section(Section { heading: section }));
            for sub_section in sub_sections {
                components.push(LatexComponent::Subsection(Subsection {
                    heading: sub_section,
                }));
            }
        }

        let commands = latex::get_commands(components)?;

        let report = latex::renderer::construct_report(
            sources,
            commands,
            "Report Title".into(),
            "Report Subtitle".into(),
        )?;

        state.state.report = Some(report.report_path);
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
        let job = GenerateReportJob;
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
        let state = job.run(state).await.unwrap();
        dbg!(state.state.report.unwrap());
    }
}
