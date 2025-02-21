use crate::prelude::*;
use async_trait::async_trait;

use super::{JobType, WorkflowState};

pub mod validation;
pub mod title;
pub mod section_names;

#[async_trait]
pub trait Job: Send + Sync + 'static {
    async fn run(&self, state: WorkflowState) -> Result<WorkflowState>;
}

impl JobType {
    pub fn next(&self) -> Option<JobType> {
        match self {
            // Start
            JobType::Pending => Some(JobType::Validation),
            JobType::Validation => Some(JobType::GenerateTitle),
            JobType::GenerateTitle => Some(JobType::GenerateSectionNames),
            // Doing
            // TODO: Add the rest of the steps
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
            _ => None,
        }
    }
}
