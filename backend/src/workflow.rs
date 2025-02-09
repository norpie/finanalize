//! ```mermaid
//! %%{init: {'theme': 'neutral', 'themeVariables': { 'primaryColor': '#e6f3ff'}}}%%
//! graph TD
//!     A[User Input] --> B{Valid Input?}
//!     B -->|Yes| C[Generate Report Title]
//!     B -->|No| Z[Error: Invalid Input]
//!     C --> D[Generate Section Headings]
//!     D --> E[[For Each Heading]]
//!     E --> F[Generate Bullet Points]
//!     F --> G[[For Each Bullet Point]]
//!     G --> H[Generate Search Queries]
//!     H --> I[[For Each Query]]
//!     I --> J[Scrape Top 5 Results]
//!     J --> K[[Process Results]]
//!     K --> L[Extract Structured Data]
//!     K --> M[Extract Unstructured Content]
//!     L --> N[Annotate Data Sources]
//!     M --> O[Annotate Content Sources]
//!     N --> P[RAG Processing]
//!     O --> P
//!     P --> Q[Generate Text Chunks]
//!     Q --> R[Combine into Coherent Paragraph]
//!     R --> S[Assemble Section Content]
//!     S --> T[[Add Citations]]
//!     T --> U[Identify Visualization Needs]
//!     U --> V[Generate/Pull Visualizations]
//!     V --> W[Finalize Section]
//!     W --> X[[Compile All Sections]]
//!     X --> Y[Generate PDF Report]
//!
//!     style A fill:#4CAF50,color:white
//!     style B fill:#FFC107,color:black
//!     style Z fill:#F44336,color:white
//!     style Y fill:#2196F3,color:white
//!     classDef loop fill:#fff8e1,stroke:#ffb300;
//!     class E,G,I,K loop;
//! ```
//!
//! A workflows describes the sequence of jobs that need to happen to complete
//! the main goal. In our case we only have one workflow, "generate report".
//! Which takes in the `user_input` and generates a financial report for it. The
//! above mermaid diagram describes our "workflow", consisting of all the "jobs"
//! that need to be done to generate a report.
//!
//! This file contains the data structures that represent the workflow and the
//! jobs.
use std::sync::Arc;

use crate::{
    db::SurrealDb, llm::LLMApi, prelude::*, scraper::BrowserWrapper, search::SearchEngine,
};

use async_trait::async_trait;

#[async_trait]
pub trait Job {
    async fn run(
        &self,
        db: Arc<SurrealDb>,
        llm: Arc<dyn LLMApi>,
        search: Arc<dyn SearchEngine>,
        browser: BrowserWrapper,
    ) -> Result<()>;
}
