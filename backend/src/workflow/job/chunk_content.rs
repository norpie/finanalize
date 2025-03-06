use crate::prelude::*;

use async_trait::async_trait;
use models::Chunk;

use crate::workflow::WorkflowState;

use super::Job;

pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Chunk {
        pub source_id: String,
        pub content: String,
    }
}

pub struct ChunkContentJob;

#[async_trait]
impl Job for ChunkContentJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let mut chunks = Vec::new();
        for source in state.state.sources.clone().unwrap().into_iter() {
            chunks.push(Chunk {
                source_id: source.id,
                content: source.content,
            });
        }
        state.state.chunks = Some(chunks);
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    use crate::{
        models::{FullReport, PreClassificationSource},
        workflow::{job::classify_sources::models::ClassifiedSource, JobType, WorkflowState},
    };

    #[tokio::test]
    #[ignore = "Uses LLM API (External Service)"]
    async fn test_classify_job_valid() {
        env_logger::init();
        let job = ChunkContentJob;
        let state = WorkflowState {
            id: "tlksajbdfaln".into(),
            last_job_type: JobType::Pending,
            state: FullReport::new("sjaudnhcrlas".into(), "Apple stock in 2025".into())
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
                .with_raw_sources(vec![
                    PreClassificationSource {
                        url: "some url".into(),
                        content:
                    r#"Apple shares rise 3% as boost in services revenue overshadows iPhone miss
=========================================================================

![Apple's Chief Executive Officer Tim Cook attends the China Development Forum in Beijing on March 24, 2024. (Photo by Pedro Pardo / AFP) (Photo by PEDRO PARDO/AFP via Getty Images)](https://image.cnbcfm.com/api/v1/image/107409413-1738273361854-107409413-1714655867152-gettyimages-2100351733-AFP_34M76UF.jpeg?v=1738273468&w=1858&h=1045&vtcrop=y)

Apple CEO Tim Cook attends the China Development Forum in Beijing on March 24, 2024.

Pedro Pardo | Afp | Getty Images

[Apple’s](https://www.cnbc.com/quotes/AAPL/) overall revenue rose 4% [in its first fiscal quarter](https://www.businesswire.com/news/home/20250130261281/en/Apple-reports-first-quarter-results), but it missed on Wall Street’s iPhone sales expectations and saw sales in China decline 11.1%, the company reported Thursday. 

But shares rose about 3% in extended trading after the company gave a forecast for the March quarter that suggested revenue growth.

Here’s how Apple did versus LSEG consensus estimates for the quarter that ended Dec. 28. 

*   **Earnings per share**: $2.40 vs. $2.35 estimated 
*   **Revenue**: $124.30 billion vs. $124.12 billion estimated 

*   **iPhone revenue**: $69.14 billion vs. $71.03 billion estimated 
*   **Mac revenue**: $8.99 billion vs. $7.96 billion estimated 
*   **iPad revenue**: $8.09 billion vs. $7.32 billion estimated 
*   **Other products revenue**: $11.75 billion vs. $12.01 billion estimated 
*   **Services revenue**: $26.34 billion vs. $26.09 billion estimated 
*   **Gross margin**: 46.9% vs. 46.5% estimated 

Apple said it expected growth in the March quarter of “low to mid single digits” on an annual basis. The company also said it expected “low double digits” growth for its Services division. Apple said it expected the strong dollar to drag on Apple’s overall sales about 2.5%, and after accounting for currency, the overall growth rate would be similar to the December quarter’s 6%.

Wall Street was expecting guidance for the March quarter of $1.66 in earnings per share on $95.46 billion in revenue. 

Apple’s profit engine, its Services division, which includes subscriptions, warranties and licensing deals, reported $23.12 billion in revenue, which is 14% higher than the same period last year. Apple CEO Tim Cook told analysts on a call Thursday that the company had more than one billion subscriptions, which includes both direct subscriptions for services such as Apple TV+ and iCloud, as well as subscriptions to third-party apps through the company’s App Store system.  

Although Apple’s overall sales rose during the quarter, the company’s closely watched iPhone sales declined slightly on a year-over-year basis. The December quarter is the first full quarter with iPhone 16 sales, and Apple released its Apple Intelligence AI suite for the devices during the quarter.  

Apple’s iPhone miss versus LSEG estimates was the biggest for the company in two years, since its first-quarter earnings report in fiscal 2023. At the time, Apple said its miss was because it was unable to make enough iPhone 14 models because of production issues in China. 

In the first fiscal quarter, the company saw significant weakness in Greater China, which includes the mainland, Hong Kong and Taiwan. Overall China sales declined 11.1% during the quarter to $18.51 billion. It is the largest drop in China sales since the same quarter last year when they fell 12.9%. 

Cook told CNBC’s Steve Kovach that iPhone sales were stronger in countries where Apple Intelligence is available. Currently, the software is only available in a handful of English-speaking countries, and it isn’t accessible in China or in Chinese. 

“During the December quarter, we saw that in markets where we had rolled out Apple intelligence, that the year-over-year performance on the iPhone 16 family was stronger than those markets where we had not rolled out Apple intelligence,” Cook said.  

He added that the company planned to release additional languages in April, including a version of Apple Intelligence in simplified Chinese.

Cook told CNBC that there were three factors in the company’s China performance. He said half of the 11.1% decline was due to a change in “channel inventory,” the fact that Apple Intelligence has not launched in the region and that after the quarter ended, China issued a national subsidy that would stimulate some Apple product sales.  

“If you look at the negative 11, half of the decline is due to a change in channel inventory, and so the operational performance is better,” Cook said. 

The company reported $36.33 billion in net income during the quarter, up 7.1% from $33.92 billion in the same period last year. 

In its fiscal first-quarter earnings report on Thursday, Apple reported a gross margin — the profit left after accounting for the cost of goods sold — of 46.9%. That is the highest on record, surpassing the 46.6% margin the company recorded in the period ending March 2024. Apple said it expected gross margin in the March quarter to be between 46.5% and 47.5%.

Apple’s iPad and Mac sales showed strong growth over last year’s struggling sales in the holiday quarter. Mac revenue rose 15% to $8.98 billion and iPad revenue grew 15% to $8.08 billion. The company’s Mac division posted its best growth since the fourth fiscal quarter of 2022.

The company released new Macs during the quarter, including the new iMac, Mac Mini and MacBook Pro laptops in October. Apple also launched a new iPad Mini during the quarter. Cook attributed the growth in those segments to new products.

“It’s driven by the significant excitement around our latest Mac lineup,” Cook said.  

Cook told analysts on an earnings call that the company had an active base of 2.35 billion active devices, up from the 2.2 billion figure the company provided a year ago.

The company’s “other products” category, also called Wearables, which includes the Apple Watch, AirPods, Beats and Vision Pro sales, declined 2% on a year-over-year basis to $11.75 billion in sales. 

Apple said it would pay a dividend of 25 cents per share and spent $30 billion on dividends and share repurchases during the first quarter.   

**WATCH:** [Apple’s superficial problem is there’s not enough demand, says Jim Cramer](https://www.cnbc.com/video/2025/01/21/apples-superficial-problem-is-theres-not-enough-demand-says-jim-cramer.html)"#.into()
                                                }
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
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.chunks.unwrap());
    }
}
