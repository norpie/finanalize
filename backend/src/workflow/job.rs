use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::prelude::*;

#[async_trait]
pub trait Job<T, U>
where
    T: Serialize + DeserializeOwned,
    U: Serialize,
{
    async fn run_ext(&self, input: String) -> Result<String> {
        let input: T = serde_json::from_str(&input)?;
        let res = self.run(input).await?;
        Ok(serde_json::to_string(&res)?)
    }

    async fn run(&self, input: T) -> Result<U>;
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

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
    impl Job<TestJobInput, TestJobOutput> for TestJob {
        async fn run(&self, input: TestJobInput) -> Result<TestJobOutput> {
            Ok(TestJobOutput {
                test: input.test.to_uppercase(),
            })
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

        assert_eq!(TestJob.run(input).await.unwrap(), expected_output);
    }
}
