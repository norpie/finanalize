use crate::prelude::*;
use async_trait::async_trait;

use super::{JobType, WorkflowState};

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
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    struct TestJobInput {
        test: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    struct TestJobOutput {
        test: String,
    }

    struct TestJob;

    #[async_trait]
    impl Job for TestJob {
        async fn run(&self, mut input: WorkflowState) -> Result<WorkflowState> {
            let state_input: TestJobInput = serde_json::from_str(&input.state)?;
            let res = TestJobOutput {
                test: state_input.test.to_uppercase(),
            };
            input.state = serde_json::to_string(&res)?;
            Ok(input)
        }
    }

    #[tokio::test]
    async fn test() {
        let input = TestJobInput {
            test: "hello".to_string(),
        };

        let input_state = WorkflowState {
            id: "aslhdasdf".into(),
            last_job_type: JobType::Pending,
            state: serde_json::to_string(&input).unwrap(),
        };

        let expected_output = TestJobOutput {
            test: "HELLO".to_string(),
        };
        assert_eq!(
            TestJob.run(input_state).await.unwrap().state,
            serde_json::to_string(&expected_output).unwrap()
        );
    }
}
