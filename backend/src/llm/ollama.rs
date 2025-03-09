use async_trait::async_trait;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use crate::prelude::*;

use std::{collections::HashMap, env};

use super::{GenerationParams, LLMApi};

#[derive(Debug, Clone)]
pub struct Ollama {
    client: reqwest::Client,
    completion_model: String,
    embed_model: String,
    base_url: String,
}

impl Default for Ollama {
    fn default() -> Self {
        let mut base_url = "http://localhost:11434".to_string();
        let base_url_opt = env::var("OLLAMA_BASE_URL").ok();
        if let Some(url) = base_url_opt {
            base_url = url;
        }
        Self {
            client: reqwest::Client::new(),
            // completion_model: "qwen2.5-coder:32b".to_string(),
            // completion_model: "llama3.2:latest".to_string(),
            completion_model: "llama3.1:latest".to_string(),
            embed_model: "nomic-embed-text".to_string(),
            base_url,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OllamaCompletionRequest<'a> {
    model: String,
    prompt: String,
    format: Option<Value>,
    stream: bool,
    options: HashMap<&'a str, Value>,
    raw: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct OllamaEmbedRequest {
    model: String,
    input: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OllamaEmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OllamaCompletionResponse {
    response: String,
}

fn default_options() -> HashMap<&'static str, Value> {
    let mut options = HashMap::new();
    options.insert("stop", Value::Array(vec!["```".into(), "</Output>".into()]));
    options.insert("num_ctx", Value::Number(Number::from_u128(12228).unwrap()));
    options.insert("keep_alive", Value::String("1m".into()));
    options.insert("temperature", Value::Number(Number::from_f64(0.5).unwrap()));
    options
}

#[async_trait]
impl LLMApi for Ollama {
    async fn generate(&self, params: &GenerationParams, prompt: String) -> Result<String> {
        let mut options = default_options();
        options.insert(
            "num_ctx",
            Value::Number(Number::from_u128(params.ctx).unwrap()),
        );
        let request = OllamaCompletionRequest {
            model: params.model.clone(),
            prompt,
            format: None,
            options,
            stream: false,
            raw: true,
        };
        debug!("Ollama request: {:?}", request.model);
        let value = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await?
            .json::<Value>()
            .await?;
        debug!("Ollama response");
        Ok(serde_json::from_value::<OllamaCompletionResponse>(value)?.response)
    }

    async fn generate_json(
        &self,
        params: &GenerationParams,
        prompt: String,
        json_schema: String,
    ) -> Result<String> {
        let mut options = default_options();
        options.insert(
            "num_ctx",
            Value::Number(Number::from_u128(params.ctx).unwrap()),
        );
        let request = OllamaCompletionRequest {
            model: params.model.clone(),
            prompt,
            options,
            stream: false,
            raw: true,
            format: Some(serde_json::from_str(&json_schema)?),
        };

        debug!("Ollama JSON request");

        let value = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await?
            .json::<Value>()
            .await?;

        debug!("Ollama JSON response");

        Ok(serde_json::from_value::<OllamaCompletionResponse>(value)?.response)
    }

    async fn embed(&self, text: String) -> Result<Vec<f32>> {
        let request = OllamaEmbedRequest {
            model: self.embed_model.clone(),
            input: text,
        };
        let value = self
            .client
            .post(format!("{}/api/embed", self.base_url))
            .json(&request)
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(serde_json::from_value::<OllamaEmbedResponse>(value)?.embeddings[0].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test_generate() {
        let ollama = Ollama::default();
        let response = ollama
            .generate(
                &GenerationParams::default(),
                "Q: How tall is the Madou Tower in Brussels?\nA:".to_string(),
            )
            .await;
        assert!(response.is_ok());
        dbg!(response.unwrap());
    }

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test_generate_json() {
        let ollama = Ollama::default();
        let response = ollama
            .generate_json(
                &GenerationParams::default(),
                "Ollama is 22 years old and is busy saving the world. Respond using JSON"
                    .to_string(),
                r#"{
            "type": "object",
            "properties": {
              "age": {
                "type": "integer"
              },
              "available": {
                "type": "boolean"
              }
            },
            "required": [
              "age",
              "available"
            ]
          }"#
                .to_string(),
            )
            .await;
        assert!(response.is_ok());
        dbg!(response.unwrap());
    }

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test_embed() {
        let ollama = Ollama::default();
        let response = ollama
            .embed(
                "What significant events shaped Apple's position in the market up to 2025?"
                    .to_string(),
            )
            .await;
        assert!(response.is_ok());
        dbg!(response.unwrap());
    }
}
