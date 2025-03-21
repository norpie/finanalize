use std::sync::Arc;

use crate::{
    llm::{GenerationParams, GenerationResult, LLMApi},
    prelude::*,
};
use handlebars::Handlebars;
use log::{debug, error, info, warn};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RetryStrategy {
    None,
    Count(usize),
    UntilSuccess,
}

/// The fix strategy to apply when the output is not valid JSON.
///
/// Types:
///     - RemoveTrailingAlphanumeric: Remove the trailing alphanumeric characters before the last
///     closing brace
///     - RemoveTrailingComma: Remove the trailing comma at the end of the JSON
///     - InsertClosedBrace: Keep inserting closing braces until the JSON is valid (max 3)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FixStrategy {
    RemoveTrailingAlphanumeric,
    RemoveTrailingComma,
    InsertClosedBrace,
}

#[derive(Debug, Clone)]
pub struct Task {
    prompt: String,
    params: GenerationParams,
    retry_strategy: RetryStrategy,
    fix_strategies: Vec<FixStrategy>,
}

#[derive(Debug, Clone)]
pub struct TaskResult<U>
where
    U: DeserializeOwned + std::fmt::Debug,
{
    pub output: U,
    pub info: GenerationResult,
}

impl Task {
    pub fn new(template: &str) -> Self {
        Self {
            prompt: template.into(),
            params: GenerationParams::default(),
            retry_strategy: RetryStrategy::Count(3),
            fix_strategies: Vec::new(),
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        debug!("Setting model to: {}", model);
        self.params.model = model;
        self
    }

    pub fn with_context_len(mut self, ctx: u128) -> Self {
        debug!("Setting context length to: {}", ctx);
        self.params.ctx = ctx;
        self
    }

    pub fn with_retry_strategy(mut self, strategy: RetryStrategy) -> Self {
        self.retry_strategy = strategy;
        self
    }

    pub fn with_fix_strategies(mut self, strategies: Vec<FixStrategy>) -> Self {
        self.fix_strategies = strategies;
        self
    }

    pub async fn run_raw<T>(&self, api: Arc<dyn LLMApi>, input: &T) -> Result<TaskResult<String>>
    where
        T: Serialize,
    {
        use std::time::Duration;
        use tokio::time::{sleep, timeout};

        let template = Handlebars::default().render_template(&self.prompt, input)?;

        loop {
            debug!("trying to generate");
            match timeout(
                Duration::from_secs(60),
                api.generate(&self.params, template.clone()),
            )
            .await
            {
                Ok(res) => {
                    return res.map(|res| TaskResult {
                        output: res.generated.clone(),
                        info: res,
                    })
                }
                Err(_) => {
                    // Timeout occurred, retry indefinitely
                    debug!("Timeout occurred, retrying indefinitely");
                    sleep(Duration::from_millis(500)).await; // Wait before retrying
                }
            }
        }
    }

    pub async fn run_structured<T, U>(
        &self,
        api: Arc<dyn LLMApi>,
        input: &T,
        schema: String,
    ) -> Result<TaskResult<U>>
    where
        T: Serialize,
        U: DeserializeOwned + std::fmt::Debug,
    {
        let prompt = Handlebars::default().render_template(&self.prompt, input)?;
        info!(
            "Starting task with retry strategy: {:?}",
            self.retry_strategy
        );
        match self.retry_strategy {
            RetryStrategy::None => self.try_run(api, prompt, schema).await,
            RetryStrategy::Count(count) => {
                let mut errors = Vec::new();
                for i in 0..count {
                    let res = self
                        .try_run::<U>(api.clone(), prompt.clone(), schema.clone())
                        .await;
                    match res {
                        Ok(value) => return Ok(value),
                        Err(err) => {
                            warn!("Task failed with: {}, retrying: {}/{}", &err, i + 1, count);
                            errors.push(err);
                        }
                    }
                }
                Err(FinanalizeError::MultipleErrors(errors))
            }
            RetryStrategy::UntilSuccess => loop {
                let res = self
                    .try_run::<U>(api.clone(), prompt.clone(), schema.clone())
                    .await;
                match res {
                    Ok(value) => return Ok(value),
                    Err(err) => {
                        error!("Task failed, retrying indefinetly: {}", err);
                    }
                }
            },
        }
    }

    async fn try_run<U>(
        &self,
        api: Arc<dyn LLMApi>,
        prompt: String,
        schema: String,
    ) -> Result<TaskResult<U>>
    where
        U: DeserializeOwned + std::fmt::Debug,
    {
        debug!("Starting generation.");
        let res = api
            .generate_json(&self.params, prompt.clone(), schema)
            .await?;
        let json = res.generated.clone();
        info!("Generated");
        let full = format!("{}{}", prompt, json);
        debug!("Parsing output.");
        let json = self.parse_output(&full)?;
        info!("Parsed output");
        debug!("Deserializing output.");
        Ok(TaskResult {
            output: self.deserialize_output(json)?,
            info: res,
        })
    }

    fn deserialize_output<U>(&self, json: String) -> Result<U>
    where
        U: DeserializeOwned + std::fmt::Debug,
    {
        debug!("Deserializing output into value.");
        let value: Value = self.deserialize_into_value(json)?;
        debug!("Deserializing value into struct.");
        dbg!(&value);
        Ok(serde_json::from_value(value)?)
    }

    fn deserialize_into_value(&self, mut json: String) -> Result<Value> {
        if self
            .fix_strategies
            .contains(&FixStrategy::RemoveTrailingAlphanumeric)
        {
            debug!("Removing trailing alphanumeric characters.");
            json = json
                .chars()
                .rev()
                .skip_while(|c| c.is_alphanumeric())
                .collect::<String>();
        }

        if self
            .fix_strategies
            .contains(&FixStrategy::RemoveTrailingComma)
        {
            debug!("Removing trailing comma.");
            json = json.trim_end_matches(',').to_string();
        }

        if !self
            .fix_strategies
            .contains(&FixStrategy::InsertClosedBrace)
        {
            info!("Skipping inserting closing brace.");
            return match serde_json::from_str(&json) {
                Ok(val) => Ok(val),
                Err(e) => {
                    error!("Failed to parse JSON: {}", json);
                    Err(e.into())
                }
            };
        }

        let mut errors = Vec::new();
        for i in 0..3 {
            let res: StdResult<Value, serde_json::Error> = serde_json::from_str(&json);
            match res {
                Ok(value) => return Ok(value),
                Err(err) => {
                    json.push('}');
                    warn!("Failed to parse JSON, retrying: {}/3: {}", i + 1, &json);
                    errors.push(err.into());
                }
            }
        }
        error!("Failed to parse JSON after 3 retries.");
        Err(FinanalizeError::MultipleErrors(errors))
    }

    fn parse_output(&self, output: &str) -> Result<String> {
        let json = output
            .lines()
            .skip_while(|line| !line.starts_with("<Output>"))
            .skip(1)
            .collect::<Vec<_>>()
            .join("\n")
            .replace("```json", "")
            .replace("```", "")
            .replace("<Output>", "")
            .replace("</Output>", "")
            .trim()
            .to_string();
        Ok(json)
    }

    fn prompt_template(&self) -> String {
        self.prompt.to_string()
    }
}

//#[cfg(test)]
//mod tests {
//    use serde::Deserialize;
//
//    use super::*;
//
//    static EXAMPLE_TASK_PROMPT_TEMPLATE: &str = r#"
//This tool extracts the city and country from a user's message. The message either contains a well-known city or it doesn't.
//If it does, the tool extracts the city and country (ISO 3166-1 alpha-3). If it doesn't, the tool returns an error message.
//
//The following are complete examples of the input and output:
//
//<Example>
//    <Input>
//    ```json
//    {
//        "message": "I am in New York"
//    }
//    ```
//    </Input>
//
//    <Output>
//    ```json
//    {
//        "city": "New York",
//        "country": "USA"
//    }
//    ```
//</Example>
//
//<Example>
//    <Input>
//    ```json
//    {
//        "message": "I am in the city"
//    }
//    ```
//    </Input>
//
//    <Output>
//    ```json
//    {
//        "error": "No city found in the message"
//    }
//    ```
//</Example>
//
//<Input>
//```json
//{
//    "message": "{{message}}"
//}
//```
//</Input>
//
//<Output>
//```json
//{"#;
//
//    #[derive(Debug, Serialize, PartialEq, Eq)]
//    struct ExampleTaskInpput {
//        message: String,
//    }
//
//    #[derive(Debug, Deserialize, PartialEq, Eq)]
//    struct ExampleTaskOutput {
//        city: String,
//        country: String,
//    }
//
//    #[tokio::test]
//    #[ignore = "Depends on external service"]
//    async fn test_example_task() {
//        let task = Task::new(EXAMPLE_TASK_PROMPT_TEMPLATE);
//
//        let input = ExampleTaskInpput {
//            message: "I am in New York".to_string(),
//        };
//
//        let output = ExampleTaskOutput {
//            city: "New York".to_string(),
//            country: "USA".to_string(),
//        };
//
//        let api = Arc::new(UllmApi::default());
//
//        let result: ExampleTaskOutput = task.run_structured(api, &input).await.unwrap();
//
//        assert_eq!(result, output);
//    }
//
//    #[derive(Debug, Deserialize, PartialEq, Eq)]
//    struct ParseOutputTest {
//        input: String,
//        expected: String,
//    }
//
//    #[tokio::test]
//    async fn test_parse_output() {
//        let task = Task::new(EXAMPLE_TASK_PROMPT_TEMPLATE);
//
//        let tests = vec![
//            ParseOutputTest {
//                input: r#"<Output>
//```json
//{
//    "city": "New York",
//    "country": "USA"
//}
//```
//</Output>"#
//                    .to_string(),
//                expected: r#"{
//    "city": "New York",
//    "country": "USA"
//}"#
//                .to_string(),
//            },
//            ParseOutputTest {
//                input: r#"<Output>
//```json
//{
//    "error": "No city found in the message"
//}
//```
//</Output>"#
//                    .to_string(),
//                expected: r#"{
//    "error": "No city found in the message"
//}"#
//                .to_string(),
//            },
//        ];
//
//        for test in tests {
//            let result = task.parse_output(&test.input).unwrap();
//            assert_eq!(result, test.expected);
//        }
//    }
//}
