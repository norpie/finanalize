use std::{env, sync::Arc};

use crate::prelude::*;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;

#[async_trait]
pub trait SearchEngine: Send + Sync + 'static {
    async fn search(&self, query: &str) -> Result<Vec<String>>;
}

pub static SEARCH: Lazy<Arc<dyn SearchEngine>> = Lazy::new(|| {
    let mut default = "http://localhost:8081".into();
    if let Ok(address) = env::var("SEARXNG_ADDRESS") {
        default = address;
    };
    Arc::new(SearxNG::new(&default))
});

#[derive(Default)]
pub struct SearxNG {
    base_url: String,
    client: Client,
}

impl SearxNG {
    pub fn new(base_url: &str) -> Self {
        SearxNG {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct SearxNGResult {
    results: Vec<SearxNGItem>,
}

#[derive(Debug, Deserialize)]
struct SearxNGItem {
    url: String,
}

#[async_trait]
impl SearchEngine for SearxNG {
    async fn search(&self, query: &str) -> Result<Vec<String>> {
        let mut urls = Vec::new();
        let url = format!(
            "{}/search?q={}&format=json&pageno={}",
            self.base_url, query, 1
        );

        let response = self.client.get(&url).send().await?;
        let results: SearxNGResult = response.json().await?;
        urls.extend(results.results.into_iter().map(|r| r.url));
        Ok(urls.into_iter().take(3).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test_searxng() {
        let results = SEARCH.search("rust").await.unwrap();
        assert!(!results.is_empty());
    }
}
