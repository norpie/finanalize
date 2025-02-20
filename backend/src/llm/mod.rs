use std::sync::Arc;

use crate::prelude::*;

use async_trait::async_trait;
use ollama::Ollama;
use once_cell::sync::Lazy;

pub mod ollama;
pub mod ullm;

pub static API: Lazy<Arc<dyn LLMApi>> = Lazy::new(|| {
    Arc::new(Ollama::default())
});

#[async_trait]
pub trait LLMApi: Send + Sync + 'static {
    /// Generate a response to a prompt, return the tokens as a string
    async fn generate(&self, prompt: String) -> Result<String>;
    async fn embed(&self, text: String) -> Result<Vec<f32>>;
}
