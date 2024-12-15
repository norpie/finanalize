use async_trait::async_trait;

#[async_trait]
trait SearchEngine {
    async fn search(&self, query: &str) -> Vec<String>;
}

#[derive(Default)]
struct YaCy;

#[async_trait]
impl SearchEngine for YaCy {
    async fn search(&self, query: &str) -> Vec<String> {
        vec![]
    }
}

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_yacy() {
        let yacy = YaCy;
        let results = yacy.search("rust").await;
        assert!(!results.is_empty());
    }
}
