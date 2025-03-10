use std::sync::Arc;

use crate::prelude::*;

use async_trait::async_trait;
use ollama::Ollama;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub mod ollama;
// pub mod ullm;

pub static API: Lazy<Arc<dyn LLMApi>> = Lazy::new(|| Arc::new(Ollama::default()));

#[derive(Debug, Clone)]
pub struct GenerationParams {
    pub model: String,
    pub ctx: u128,
}

impl Default for GenerationParams {
    fn default() -> Self {
        Self {
            model: "llama3.1:latest".to_string(),
            ctx: 12228,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerationCaching {
    None,
    OpenAI {
        cached_input_token_count: usize,
    },
    Anthropic {
        prompt_caching_read_token_count: usize,
        prompt_caching_write_token_count: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Api {
    Ollama,
    OpenAI,
    Anthropic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    pub generated: String,
    pub api: Api,
    pub prompt_token_count: usize,
    pub generated_token_count: usize,
    pub caching: GenerationCaching,
    pub total_duration_us: i64,
}

impl From<GenerationResult> for String {
    fn from(result: GenerationResult) -> Self {
        result.generated
    }
}

#[async_trait]
pub trait LLMApi: Send + Sync + 'static {
    /// Generate a response to a prompt, return the tokens as a string
    async fn generate(&self, params: &GenerationParams, prompt: String)
        -> Result<GenerationResult>;
    async fn generate_json(
        &self,
        params: &GenerationParams,
        prompt: String,
        json_schema: String,
    ) -> Result<GenerationResult>;

    async fn embed(&self, text: String) -> Result<Vec<f32>>;
}
