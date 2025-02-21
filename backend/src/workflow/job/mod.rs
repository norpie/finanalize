use crate::prelude::*;
use async_trait::async_trait;

use super::{JobType, WorkflowState};

pub mod validation;
pub mod title;

#[async_trait]
pub trait Job: Send + Sync + 'static {
    async fn run(&self, input: WorkflowState) -> Result<WorkflowState>;
}

impl JobType {
    pub fn next(&self) -> Option<JobType> {
        match self {
            // Start
            JobType::Pending => Some(JobType::Validation),
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
            _ => None,
        }
    }
}
