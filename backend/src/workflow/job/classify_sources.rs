pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ClassifySourcesInput {
        pub input: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ClassifySourcesOutput {
        pub title: String,
        pub summary: String,
        pub author: String,
        pub date: Option<String>,
        #[serde(rename = "publishedAfter")]
        pub published_after: Option<String>,
    }
}

