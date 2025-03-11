use async_trait::async_trait;
use log::debug;
use models::EmbeddedChunk;

use crate::db::DB;
use crate::llm::API;
use crate::prelude::*;

use crate::workflow::WorkflowState;

use super::Job;

pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EmbeddedChunk {
        pub report_id: String,
        pub source_id: String,
        pub chunk: String,
        pub embeddings: Vec<f32>,
    }
}

pub struct IndexChunksJob;

#[async_trait]
impl Job for IndexChunksJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let mut chunks_embeddings = vec![];
        let chunks = state.state.chunks.clone().unwrap();
        let len = chunks.len();
        debug!("Indexing chunks");
        for (i, chunk) in chunks.iter().enumerate() {
            debug!("Indexing chunk({}/{}): {}", i, len, chunk.source_id);
            let embeddings = API.clone().embed(chunk.content.clone()).await?;
            let embedded_chunk = EmbeddedChunk {
                report_id: state.state.id.clone(),
                source_id: chunk.source_id.clone(),
                chunk: chunk.content.clone(),
                embeddings,
            };
            chunks_embeddings.push(embedded_chunk.clone());
            let _: Option<EmbeddedChunk> = DB
                .get()
                .unwrap()
                .create("embedded_chunk")
                .content(embedded_chunk)
                .await?;
        }
        state.state.chunk_embeddings = Some(chunks_embeddings);
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    use crate::{
        models::FullReport,
        workflow::{
            job::{chunk_content::models::Chunk, classify_sources::models::ClassifiedSource},
            JobType, WorkflowState,
        },
    };

    #[tokio::test]
    #[ignore = "Uses LLM API (External Service)"]
    async fn test_classify_job_valid() {
        env_logger::init();
        let job = IndexChunksJob;
        let state = WorkflowState {
            id: "asdlfjhasldfjh".into(),
            last_job_type: JobType::Pending,
            state: FullReport::new("asdlfjhasldfjh".into(), "Apple stock in 2025".into())
                .with_title("State of Apple in 2025".into())
                .with_sections(vec![
                    "Introduction".into(),
                    "Market Analysis".into(),
                    "Financial Analysis".into(),
                    "Conclusion".into(),
                ])
                .with_sub_sections(vec![
                    vec!["Background".into(), "Problem Statement".into()],
                    vec!["Market Size".into(), "Market Share".into()],
                    vec!["Revenue".into(), "Profit".into()],
                    vec!["Recommendation".into()],
                ])
                .with_searches(vec![
                    "background on apple company 2025".into(),
                    "history of apple corporation 2025".into(),
                    "origins of apple technology 2025".into(),
                    "apple problem statement 2025".into(),
                    "challenges faced by apple in 2025".into(),
                    "issues affecting apple business in 2025".into(),
                    "apple market size forecast 2025".into(),
                    "growth projection for apple market 2025".into(),
                    "expected apple market value 2025".into(),
                    "apple market share analysis 2025".into(),
                    "market position of apple in 2025".into(),
                    "apple's share in global tech market 2025".into(),
                    "revenue trends for apple 2025".into(),
                    "apple financial performance revenue 2025".into(),
                    "annual revenue forecast for apple 2025".into(),
                    "profit analysis of apple 2025".into(),
                    "net profit forecast for apple 2025".into(),
                    "apple's profitability in 2025".into(),
                ])
                .with_search_results(vec![
                    "https://backlinko.com/apple-statistics".into(),
                    "https://blog.tbrc.info/2025/02/apples-market-demand/".into(),
                    "https://capital.com/en-eu/analysis/apple-stock-price-in-10-years".into(),
                    "https://coincodex.com/stock/AAPL/price-prediction/".into(),
                    "https://cyble.com/blog/apple-fixes-cve-2025-24085-security-update/".into(),
                    "https://www.businessofapps.com/data/apple-statistics/".into(),
                    "https://www.captide.co/insights/apple-q1-2025".into(),
                    "https://www.cnbc.com/2025/01/30/apple-aapl-q1-earnings-2025.html".into(),
                    "https://www.cultofmac.com/apple-history/apple-incorporation".into(),
                    "https://www.nasdaq.com/articles/history-apple-company-and-stock".into(),
                    "https://www.nasdaq.com/articles/what-lies-ahead-apple-stock-etfs-2025".into(),
                    "https://www.officetimeline.com/blog/apple-inc-timeline".into(),
                    "https://www.technavio.com/report/fresh-apples-market-industry-analysis".into(),
                ])
                .with_sources(vec![
                    ClassifiedSource {
                        id: "0".into(),
                        content: r#"# Apple shares rise 3% as boost in services revenue overshadows iPhone miss

> Kif Leswing, CNBC
> Published on January 30, 2025

Apple's overall revenue rose 4% during its first fiscal quarter, but it missed on Wall Street's iPhone sales expectations and saw sales in China decline 11.1%, the company reported Thursday.

Although Apple's overall sales rose during the quarter, the company's closely watched iPhone sales declined slightly on a year-over-year basis. The December quarter is the first full quarter with iPhone 16 sales, and Apple released its Apple Intelligence AI suite for the devices during the quarter.

Apple's profit engine, its Services division, which includes subscriptions, warranties and licensing deals, reported $23.12 billion in revenue, which is 14% higher than the same period last year. Apple CEO Tim Cook told analysts on a call Thursday that the company had more than one billion subscriptions, which includes both direct subscriptions for services such as Apple TV+ and iCloud, as well as subscriptions to third-party apps through the company's App Store system.

The December quarter is the first full quarter with iPhone 16 sales, and Apple released its Apple Intelligence AI suite for the devices during the quarter.

Apple said it expected growth in the March quarter of "low to mid single digits" on an annual basis. The company also said it expected "low double digits" growth for its Services division.

## Secondary numbers
- $2.40 - Earnings per share
- $124.30 billion - Revenue
- $69.14 billion - iPhone revenue
- $8.99 billion - Mac revenue
- $8.09 billion - iPad revenue
- $11.75 billion - Other products revenue
- $26.34 billion - Services revenue
- 46.9% - Gross margin"#.into(),
                        url: "https://www.nbcboston.com/news/business/money-report/apple-reports-first-quarter-earnings-after-the-bell-2/3617779/?os=android&ref=app&noamp=mobile".into(),
                        title: "Apple shares rise 3% as boost in services revenue overshadows iPhone miss".into(),
                        author: "Kif Leswing, CNBC".into(),
                        published_after: Some(Utc::now().format("%Y-%m-%d").to_string()),
                        date: Some(Utc::now().format("%Y-%m-%d").to_string()),
                    }
                ])
                .with_chunks(vec![
    Chunk {
        source_id: "0".into(),
        content: r#"# Apple shares rise 3% as boost in services revenue overshadows iPhone miss

> Kif Leswing, CNBC
> Published on January 30, 2025

Apple's overall revenue rose 4% during its first fiscal quarter, but it missed on Wall Street's iPhone sales expectations and saw sales in China decline 11.1%, the company reported Thursday.

Although Apple's overall sales rose during the quarter, the company's closely watched iPhone sales declined slightly on a year-over-year basis. The December quarter is the first full quarter with iPhone 16 sales, and Apple released its Apple Intelligence AI suite for the devices during the quarter.

Apple's profit engine, its Services division, which includes subscriptions, warranties and licensing deals, reported $23.12 billion in revenue, which is 14% higher than the same period last year. Apple CEO Tim Cook told analysts on a call Thursday that the company had more than one billion subscriptions, which includes both direct subscriptions for services such as Apple TV+ and iCloud, as well as subscriptions to third-party apps through the company's App Store system.

The December quarter is the first full quarter with iPhone 16 sales, and Apple released its Apple Intelligence AI suite for the devices during the quarter.

Apple said it expected growth in the March quarter of "low to mid single digits" on an annual basis. The company also said it expected "low double digits" growth for its Services division.

## Secondary numbers
- $2.40 - Earnings per share
- $124.30 billion - Revenue
- $69.14 billion - iPhone revenue
- $8.99 billion - Mac revenue
- $8.09 billion - iPad revenue
- $11.75 billion - Other products revenue
- $26.34 billion - Services revenue
- 46.9% - Gross margin"#.into(),
    }])
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.chunk_embeddings.unwrap());
    }
}
