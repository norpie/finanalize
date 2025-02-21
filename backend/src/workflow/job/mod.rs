use crate::prelude::*;
use async_trait::async_trait;

use super::{JobType, WorkflowState};

pub mod validation;
pub mod title;
pub mod section_names;
pub mod sub_sections;

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
            JobType::GenerateSectionNames => Some(JobType::GenerateSubSections),
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
            _ => None,
        }
    }
}
