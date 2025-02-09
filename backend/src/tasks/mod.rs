use std::sync::Arc;

use crate::{llm::LLMApi, prelude::*};
use handlebars::Handlebars;
use serde::{de::DeserializeOwned, Serialize};

pub struct Task<'a>(&'a str);

impl<'a> Task<'a> {
    pub fn new(template: &'a str) -> Self {
        Self(template)
    }

    pub async fn run<T, U>(&self, api: Arc<dyn LLMApi>, input: &T) -> Result<U>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let prompt = Handlebars::default().render_template(self.0, input)?;
        let generated = api.generate(prompt.clone()).await?;
        let full = format!("{}{}", prompt, generated);
        let json = self.parse_output(&full)?;
        let output: U = serde_json::from_str(&json)?;
        Ok(output)
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
        self.0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::llm::ullm::UllmApi;

    use super::*;

    static EXAMPLE_TASK_PROMPT_TEMPLATE: &str = r#"
This tool extracts the city and country from a user's message. The message either contains a well-known city or it doesn't.
If it does, the tool extracts the city and country (ISO 3166-1 alpha-3). If it doesn't, the tool returns an error message.

The following are complete examples of the input and output:

<Example>
    <Input>
    ```json
    {
        "message": "I am in New York"
    }
    ```
    </Input>

    <Output>
    ```json
    {
        "city": "New York",
        "country": "USA"
    }
    ```
</Example>

<Example>
    <Input>
    ```json
    {
        "message": "I am in the city"
    }
    ```
    </Input>

    <Output>
    ```json
    {
        "error": "No city found in the message"
    }
    ```
</Example>

<Input>
```json
{
    "message": "{{message}}"
}
```
</Input>

<Output>
```json
{"#;

    #[derive(Debug, Serialize, PartialEq, Eq)]
    struct ExampleTaskInpput {
        message: String,
    }

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct ExampleTaskOutput {
        city: String,
        country: String,
    }

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test_example_task() {
        let task = Task::new(EXAMPLE_TASK_PROMPT_TEMPLATE);

        let input = ExampleTaskInpput {
            message: "I am in New York".to_string(),
        };

        let output = ExampleTaskOutput {
            city: "New York".to_string(),
            country: "USA".to_string(),
        };

        let api = Arc::new(UllmApi::default());

        let result: ExampleTaskOutput = task.run(api, &input).await.unwrap();

        assert_eq!(result, output);
    }

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct ParseOutputTest {
        input: String,
        expected: String,
    }

    #[tokio::test]
    async fn test_parse_output() {
        let task = Task::new(EXAMPLE_TASK_PROMPT_TEMPLATE);

        let tests = vec![
            ParseOutputTest {
                input: r#"<Output>
```json
{
    "city": "New York",
    "country": "USA"
}
```
</Output>"#
                    .to_string(),
                expected: r#"{
    "city": "New York",
    "country": "USA"
}"#
                .to_string(),
            },
            ParseOutputTest {
                input: r#"<Output>
```json
{
    "error": "No city found in the message"
}
```
</Output>"#
                    .to_string(),
                expected: r#"{
    "error": "No city found in the message"
}"#
                .to_string(),
            },
        ];

        for test in tests {
            let result = task.parse_output(&test.input).unwrap();
            assert_eq!(result, test.expected);
        }
    }
}
