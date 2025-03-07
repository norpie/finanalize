use std::sync::Arc;

use crate::prelude::*;

use async_trait::async_trait;
use ollama::Ollama;
use once_cell::sync::Lazy;

pub mod ollama;
// pub mod ullm;

pub static API: Lazy<Arc<dyn LLMApi>> = Lazy::new(|| Arc::new(Ollama::default()));

#[derive(Debug, Clone)]
pub struct GenerationParams {
    model: String,
    ctx: u128,
}

impl Default for GenerationParams {
    fn default() -> Self {
        Self {
            model: "llama3.1:latest".to_string(),
            ctx: 12228,
        }
    }
}

#[async_trait]
pub trait LLMApi: Send + Sync + 'static {
    /// Generate a response to a prompt, return the tokens as a string
    async fn generate(&self, prompt: String) -> Result<String>;
    async fn generate_paramed(&self, params: GenerationParams, prompt: String) -> Result<String>;
    async fn generate_json(&self, prompt: String, json_schema: String) -> Result<String>;
    async fn generate_json_paramed(&self, params: GenerationParams, prompt: String, json_schema: String) -> Result<String>;
    async fn embed(&self, text: String) -> Result<Vec<f32>>;
}
