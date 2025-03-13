use std::{str::FromStr, sync::Arc};

use crate::prelude::*;

use async_trait::async_trait;
use ollama::Ollama;
use once_cell::sync::Lazy;
use rust_decimal::Decimal;
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
pub enum CostType {
    Input,
    Output,
    CachedInput,
    CacheRead,
    CacheWrite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Api {
    Ollama,
    OpenAI,
    Anthropic,
}

impl Api {
    /// Exchange rate between type and credits
    pub fn exchange_rate(&self, ct: CostType) -> Decimal {
        match self {
            Api::Ollama => match ct {
                CostType::Input => Decimal::from_str("0.0005").unwrap(),
                CostType::Output => Decimal::from_str("0.0015").unwrap(),
                _ => Decimal::from(0),
            }
            // Credits are 1000 per $1
            //
            // Input: $0.50 per 1M tokens, so in credits it's 500 credits per 1M tokens, or 0.0005
            // credits per token
            // Output: $1.50 per 1M tokens, so in credits it's 1500 credits per 1M tokens, or
            // 0.0015
            Api::OpenAI => match ct {
                CostType::Input => Decimal::from_str("0.0005").unwrap(),
                CostType::Output => Decimal::from_str("0.0015").unwrap(),
                _ => Decimal::from(0),
            },
            _ => Decimal::from(0),
        }
    }

    pub fn cost(self, gr: GenerationResult) -> Decimal {
        let mut total = Decimal::ZERO;
        let input = Decimal::from_str(&gr.prompt_token_count.to_string())
            .unwrap()
            .saturating_mul(self.exchange_rate(CostType::Input));
        let output = Decimal::from_str(&gr.generated_token_count.to_string())
            .unwrap()
            .saturating_mul(self.exchange_rate(CostType::Output));
        let cache_cost = match gr.caching {
            GenerationCaching::Anthropic {
                prompt_caching_read_token_count,
                prompt_caching_write_token_count,
            } => Decimal::from_str(&prompt_caching_read_token_count.to_string())
                .unwrap()
                .saturating_mul(self.exchange_rate(CostType::CacheRead))
                .saturating_add(
                    Decimal::from_str(&prompt_caching_write_token_count.to_string())
                        .unwrap()
                        .saturating_mul(self.exchange_rate(CostType::CacheWrite)),
                ),
            GenerationCaching::OpenAI {
                cached_input_token_count,
            } => Decimal::from_str(&cached_input_token_count.to_string())
                .unwrap()
                .saturating_mul(self.exchange_rate(CostType::CachedInput)),
            GenerationCaching::None => Decimal::ZERO,
        };
        total += input;
        total += output;
        total += cache_cost;
        total
    }
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
