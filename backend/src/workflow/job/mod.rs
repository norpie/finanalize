use crate::prelude::*;
use async_trait::async_trait;

use super::JobType;

#[async_trait]
pub trait Job: Send + Sync + 'static {
    async fn run(&self, input: String) -> Result<String>;
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
        async fn run(&self, input: String) -> Result<String> {
            let input: TestJobInput = serde_json::from_str(&input)?;
            let res = TestJobOutput {
                test: input.test.to_uppercase(),
            };
            Ok(serde_json::to_string(&res)?)
        }
    }

    #[tokio::test]
    async fn test() {
        let input = TestJobInput {
            test: "hello".to_string(),
        };

        let expected_output = TestJobOutput {
            test: "HELLO".to_string(),
        };
        assert_eq!(
            TestJob
                .run(serde_json::to_string(&input).unwrap())
                .await
                .unwrap(),
            serde_json::to_string(&expected_output).unwrap()
        );
    }
}
