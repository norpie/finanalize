use crate::prelude::*;

use super::LLMApi;

use async_trait::async_trait;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UllmModel {
    engine: String,
    name: String,
}

#[derive(Debug)]
pub struct UllmApi {
    client: Client,
    base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UllmCompletionRequest {
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UllmCompletionResponse {
    completion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UllmStatus {
    status: String,
    engine: Option<String>,
    model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UllmStatusName {
    #[serde(rename = "loaded")]
    Loaded,
    #[serde(rename = "unloaded")]
    Unloaded,
    #[serde(rename = "no_model")]
    NoModel,
}

impl Default for UllmApi {
    fn default() -> Self {
        Self {
            client: Default::default(),
            base_url: "http://localhost:8082".to_string(),
        }
    }
}

impl UllmApi {
    pub async fn list(&self) -> Result<Vec<UllmModel>> {
        let url = format!("{}/models", self.base_url);
        let response = self.client.get(&url).send().await?;
        let models = response.json::<Vec<UllmModel>>().await?;
        Ok(models)
    }

    pub async fn status(&self) -> Result<UllmStatus> {
        let url = format!("{}/status", self.base_url);
        let response = self.client.get(&url).send().await?;
        let status = response.json::<UllmStatus>().await?;
        Ok(status)
    }

    pub async fn unload(&self) -> Result<()> {
        let url = format!("{}/models", self.base_url);
        let response = self.client.delete(&url).send().await?;
        if !response.status().is_success() {
            return Err(FinanalizeError::LlmApi(format!(
                "Failed to unload models: {}",
                response.text().await?
            )));
        }
        Ok(())
    }

    pub async fn load(&self, model: UllmModel) -> Result<()> {
        let url = format!("{}/models/{}/{}", self.base_url, model.engine, model.name);
        let response = self.client.post(&url).send().await?;
        if !response.status().is_success() {
            return Err(FinanalizeError::LlmApi(format!(
                "Failed to load model: {}",
                response.text().await?
            )));
        }
        Ok(())
    }
}

static DEFAULT_MODEL: Lazy<UllmModel> = Lazy::new(|| UllmModel {
    engine: "EXLLAMAV2".into(),
    name: "bartowski_Qwen2.5-Coder-32B-Instruct-exl2_4_25".into(),
});

#[async_trait]
impl LLMApi for UllmApi {
    async fn generate(&self, prompt: String) -> Result<String> {
        let status = self.status().await?;
        if status.status == "unloaded" || status.status == "no_model" {
            self.load(DEFAULT_MODEL.clone()).await?;
        }
        let value: Value = self
            .client
            .post(format!("{}/complete", self.base_url))
            .json(&UllmCompletionRequest { text: prompt })
            .send()
            .await?
            .json()
            .await?;
        dbg!(&value);
        let completion: UllmCompletionResponse = serde_json::from_value(value)?;
        Ok(completion.completion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new() {
        let api = UllmApi::default();
        dbg!(&api);
        // api.unload().await.unwrap();
        let list = api.list().await.unwrap();
        dbg!(&list);
        // api.load(DEFAULT_MODEL.clone()).await.unwrap();
        let prompt = "Question: How tall is the Brussels Madou tower?\nAnswer:".to_string();
        let generated = api.generate(prompt.clone()).await.unwrap();
        println!("generated: {}", generated);
        println!("full message: {}", prompt + &generated);
    }
}
