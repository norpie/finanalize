use crate::prelude::*;
use async_trait::async_trait;

use super::{JobType, WorkflowState};

pub mod validation;
pub mod title;
pub mod section_names;
pub mod sub_sections;
pub mod search_queries;

#[async_trait]
pub trait Job: Send + Sync + 'static {
    async fn run(&self, state: WorkflowState) -> Result<WorkflowState>;
}

/*
[
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
 */

impl JobType {
    pub fn next(&self) -> Option<JobType> {
        match self {
            // Start
            JobType::Pending => Some(JobType::Validation),
            // Doing
            JobType::Validation => Some(JobType::GenerateTitle),
            JobType::GenerateTitle => Some(JobType::GenerateSectionNames),
            JobType::GenerateSectionNames => Some(JobType::GenerateSubSections),
            JobType::GenerateSubSections => Some(JobType::GenerateSearchQueries),
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
            JobType::GenerateSubSections => Some(Box::new(sub_sections::SubSectionsJob)),
            JobType::GenerateSearchQueries => Some(Box::new(search_queries::GenerateSearchQueriesJob)),
            _ => None,
        }
    }
}
