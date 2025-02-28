use crate::prelude::*;
use async_trait::async_trait;

use super::{JobType, WorkflowState};

pub mod generate_report;
pub mod scrape_pages;
pub mod search_queries;
pub mod search_terms;
pub mod section_names;
pub mod sub_sections;
pub mod title;
pub mod validation;
pub mod classify_sources;

#[async_trait]
pub trait Job: Send + Sync + 'static {
    async fn run(&self, state: WorkflowState) -> Result<WorkflowState>;
}

impl JobType {
    pub fn next(&self) -> Option<JobType> {
        match self {
            // Start
            JobType::Pending => Some(JobType::Validation),
            // Doing
            JobType::Validation => Some(JobType::GenerateTitle),
            JobType::GenerateTitle => Some(JobType::GenerateSectionNames),
            JobType::GenerateSectionNames => Some(JobType::GenerateSubSectionNames),
            JobType::GenerateSubSectionNames => Some(JobType::GenerateSearchQueries),
            JobType::GenerateSearchQueries => Some(JobType::SearchQueries),
            JobType::SearchQueries => Some(JobType::ScrapeTopResults),
            JobType::ScrapeTopResults => Some(JobType::ExtractContent), // TODO: Implement
                                                                        // ExtractContent
            JobType::ExtractContent => Some(JobType::ClassifyContent),
            JobType::ClassifyContent => Some(JobType::Done), // TODO: Add more steps
            // JobType::GeneratePDFReport => Some(JobType::Done),
            // Done
            JobType::Invalid => None,
            JobType::Done => None,
            _ => None,
        }
    }

    pub fn job(&self) -> Option<Box<dyn Job>> {
        match self {
            JobType::Pending => None,
            JobType::Validation => Some(Box::new(validation::ValidationJob)),
            JobType::GenerateTitle => Some(Box::new(title::TitleJob)),
            JobType::GenerateSectionNames => Some(Box::new(section_names::SectionNamesJob)),
            JobType::GenerateSubSectionNames => Some(Box::new(sub_sections::SubSectionsJob)),
            JobType::GenerateSearchQueries => {
                Some(Box::new(search_queries::GenerateSearchQueriesJob))
            }
            JobType::SearchQueries => Some(Box::new(search_terms::SearchJob)),
            JobType::ScrapeTopResults => Some(Box::new(scrape_pages::ScrapePagesJob)),
            JobType::ExtractContent => Some(Box::new(classify_sources::ClassifySourcesJob)),
            JobType::RenderLaTeXPdf => Some(Box::new(generate_report::GenerateReportJob)),
            _ => None,
        }
    }
}
