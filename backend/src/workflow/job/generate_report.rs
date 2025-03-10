use async_trait::async_trait;
use itertools::izip;
use log::debug;

use crate::latex::{self, LatexComponent, Section, Source, Subsection};
use crate::prelude::*;

use crate::workflow::WorkflowState;

use super::Job;

pub mod models {}

pub struct GenerateReportJob;

#[async_trait]
impl Job for GenerateReportJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        debug!("Running GenerateReportJob...");
        let mut components = Vec::new();
        debug!(
            "Generating report for: {}",
            state.state.title.clone().unwrap()
        );
        for (section_name, sub_sections, sub_section_contents) in izip!(
            state.state.sections.clone().unwrap().into_iter(),
            state.state.sub_sections.clone().unwrap().into_iter(),
            state
                .state
                .sub_section_contents
                .clone()
                .unwrap()
                .into_iter(),
        ) {
            components.push(LatexComponent::Section(Section {
                heading: section_name,
            }));
            for (sub_section_name, sub_section_content) in
                sub_sections.into_iter().zip(sub_section_contents)
            {
                components.push(LatexComponent::Subsection(Subsection {
                    heading: sub_section_name,
                }));
                components.push(LatexComponent::Text(sub_section_content));
            }
        }

        let mut sources = vec![];
        for source in state.state.sources.clone().unwrap().into_iter() {
            sources.push(Source::new(
                "Website".into(),
                source.id,
                source.author,
                source.title,
                2025,
                "Journal".into(),
                source.url,
            ));
        }

        let commands = latex::get_commands(components)?;
        debug!("Received {:?} commands", commands.len());
        let report = latex::renderer::construct_report(
            sources,
            commands,
            state.state.title.clone().unwrap(),
            " ".into(),
        )?;

        debug!("Report can be found at: {}", &report.report_path);
        state.state.report = Some(report.report_path);
        debug!("GenerateReportJob completed");
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
                .with_sources(Vec::new()),
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.report.unwrap());
    }
}
