use crate::prelude::*;

use async_trait::async_trait;

#[async_trait]
pub trait LLMApi: Send + Sync + 'static {
    /// Generate a response to a prompt, return the tokens as a string
    async fn generate(&self, prompt: String) -> Result<String>;
}

pub mod ullm;
