use crate::llm::API;
use crate::prelude::*;
use crate::tasks::Task;
use crate::workflow::job::Job;
use crate::workflow::WorkflowState;
use crate::{graphing, prompting};
use async_trait::async_trait;
use log::debug;

pub struct GenerateGraphsJob;

pub mod models {
    use crate::graphing::{GraphData, HistogramData, PieChartData, StockChartData};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct GraphInput {
        pub graph_type: String,
        pub text_id: String,
        pub data: String,
        pub purpose: String,
        pub x_label: String,
        pub y_label: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct GraphFileOutput {
        pub graph_type: String,
        pub file_path: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct LineDataOutput {
        pub graph_data: GraphData,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct BarDataOutput {
        pub graph_data: HistogramData,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PieDataOutput {
        pub graph_data: PieChartData,
    }
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct StockDataOutput {
        pub graph_data: StockChartData,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct TableDataOutput {
        pub graph_data: TableOutput,    }
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct TableOutput {
        pub caption: String,
        pub rows: Vec<Vec<String>>,
        pub columns: Vec<String>,
    }
}

#[async_trait]
impl Job for GenerateGraphsJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        debug!("Running GenerateGraphsJob...");
        let mut charts = Vec::new();
        let mut tables = Vec::new();
        if let Some(graphics) = state.state.graphics.clone() {
            for graphic in graphics {
                let prompt = prompting::get_prompt("graph-data-prep".to_string())?;
                let task = Task::new(&prompt);
                let input = models::GraphInput {
                    graph_type: graphic.graph_type.clone(),
                    text_id: graphic.text_id.clone(),
                    data: graphic.data_used.clone(),
                    purpose: graphic.purpose.clone(),
                    x_label: graphic.x_label.clone(),
                    y_label: graphic.y_label.clone(),
                };
                debug!("Prepared input: {:#?}", input);
                debug!("Running task...");
                match graphic.graph_type.as_str() {
                    "line" => {
                        let output: models::LineDataOutput =
                            task.run_structured(API.clone(), &input).await?;
                        let chart = graphing::create_graph(
                            "line".to_string(),
                            Some(output.graph_data),
                            None,
                            None,
                            None,
                        )
                        .expect("Could not create line/scatter graph");
                        let graph_file_output = models::GraphFileOutput {
                            graph_type: chart.chart_type.clone(),
                            file_path: chart.chart_file.clone(),
                        };
                        charts.push(graph_file_output);
                    }
                    "bar" => {
                        let output: models::BarDataOutput =
                            task.run_structured(API.clone(), &input).await?;
                        let chart = graphing::create_graph(
                            "bar".to_string(),
                            None,
                            Some(output.graph_data),
                            None,
                            None,
                        )
                        .expect("Could not create histogram");
                        let graph_file_output = models::GraphFileOutput {
                            graph_type: chart.chart_type.clone(),
                            file_path: chart.chart_file.clone(),
                        };
                        charts.push(graph_file_output);
                    }
                    "pie" => {
                        let output: models::PieDataOutput =
                            task.run_structured(API.clone(), &input).await?;
                        let chart = graphing::create_graph(
                            "pie".to_string(),
                            None,
                            None,
                            Some(output.graph_data),
                            None,
                        )
                        .expect("Could not create pie chart");
                        let graph_file_output = models::GraphFileOutput {
                            graph_type: chart.chart_type.clone(),
                            file_path: chart.chart_file.clone(),
                        };
                        charts.push(graph_file_output);
                    }
                    "stock" => {
                        let output: models::StockDataOutput =
                            task.run_structured(API.clone(), &input).await?;
                        let chart = graphing::create_graph(
                            "stock".to_string(),
                            None,
                            None,
                            None,
                            Some(output.graph_data),
                        )
                        .expect("Could not create stock graph");
                        let graph_file_output = models::GraphFileOutput {
                            graph_type: chart.chart_type.clone(),
                            file_path: chart.chart_file.clone(),
                        };
                        charts.push(graph_file_output);
                    }
                    "table" => {
                        let output: models::TableDataOutput =
                            task.run_structured(API.clone(), &input).await?;
                        tables.push(output.graph_data);
                    }
                    _ => {}
                }
                debug!("Task completed");
            }
            state.state.charts = Some(charts);
            debug!("Charts: {:#?}", state.state.charts);
            state.state.tables = Some(tables);
            debug!("Tables: {:#?}", state.state.tables);
            dbg!(&state.state.report);
        }
        debug!("GenerateGraphsJob completed");
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::workflow::job::graphic_identifier::models::{Graphic, Text};
    use crate::{
        models::FullReport,
        workflow::{JobType, WorkflowState},
    };
    use crate::models::FullReport;

    #[tokio::test]
    // #[ignore = "Uses LLM API (External Service)"]
    async fn test_generate_graphs_job() {
        env_logger::init();
        dotenvy::from_filename(".env").ok();
        let job = GenerateGraphsJob;
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
                ])
                .with_texts(vec![
                    Text {
                        id: "a".to_string(),
                        text: r#"Apple’s overall revenue rose 4% in its first fiscal quarter, reaching $124.3 billion. However, the company missed Wall Street’s iPhone sales expectations, with revenue from iPhone sales declining to $69.14 billion compared to the estimated $71.03 billion. This shortfall, coupled with an 11.1% decline in sales within the Chinese market, raised concerns about Apple’s growth trajectory in key international regions."#.to_string(),
                    },
                    Text {
                        id: "b".to_string(),
                        text: "Despite challenges in certain product segments, Apple reported a strong net income of $36.33 billion for the quarter, reflecting a 7.1% increase from the $33.92 billion recorded in the same period last year. The company's ability to maintain profitability, even in a highly competitive market, is attributed to its diversified revenue streams, including its growing Services division.".to_string(),
                    },
                    Text {
                        id: "c".to_string(),
                        text: "Apple’s Services division continues to be its most robust revenue generator, bringing in $23.12 billion during the quarter—14% higher than the same period in the previous year. This segment includes revenue from subscriptions, licensing deals, and warranties, showcasing the increasing importance of digital services in Apple's overall business strategy.".to_string(),
                    },
                    Text {
                        id: "d".to_string(),
                        text: "Revenue from Mac and iPad sales saw a significant rise, growing 15% each compared to last year’s struggling holiday quarter. Mac revenue climbed to $8.98 billion, while iPad revenue reached $8.08 billion. This growth is largely attributed to the launch of new MacBook Pro and iMac models, along with the release of a refreshed iPad Mini.".to_string(),
                    },
                    Text {
                        id: "e".to_string(),
                        text: "Apple reported a gross margin of 46.9%, its highest on record, surpassing the previous record of 46.6% set in March 2024. This improvement in profitability is a result of cost optimizations, higher-margin service offerings, and an overall increase in product pricing strategies.".to_string(),
                    },
                    Text {
                        id: "f".to_string(),
                        text: "While financial performance showed resilience, investor sentiment remains mixed due to concerns over slowing iPhone sales and increased competition in key global markets. Analysts speculate that demand for premium smartphones has softened, prompting Apple to explore new markets and product categories to sustain long-term growth.".to_string(),
                    },
                    Text {
                        id: "g".to_string(),
                        text: "Apple’s market position remains strong despite declining sales in China. CEO Tim Cook attributed the downturn in China to factors including inventory adjustments and the delayed rollout of Apple Intelligence features in non-English-speaking markets. The company aims to address this gap by expanding language support and launching targeted marketing campaigns.".to_string(),
                    },
                    Text {
                        id: "h".to_string(),
                        text: "The company's 'other products' category, which includes the Apple Watch, AirPods, and the recently launched Vision Pro headset, recorded $11.75 billion in revenue. However, this represents a 2% decline from the previous year, suggesting potential saturation in the wearables market or shifts in consumer spending habits.".to_string(),
                    },
                    Text {
                        id: "i".to_string(),
                        text: "In its guidance for the next quarter, Apple projected revenue growth in the 'low to mid single digits,' with a stronger emphasis on expanding its digital ecosystem. The company also anticipates 'low double-digit' growth for its Services division, driven by increasing subscriptions to platforms like Apple Music, Apple TV+, and iCloud.".to_string(),
                    },
                    Text {
                        id: "j".to_string(),
                        text: "Beyond financials, Apple continues to emphasize sustainability and environmental responsibility. The company announced plans to further reduce its carbon footprint by implementing recycled materials in new product lines and expanding its use of renewable energy in its supply chain.".to_string(),
                    },
                    Text {
                        id: "k".to_string(),
                        text: "Apple's stock price remained relatively stable following the earnings announcement, with a slight uptick of 3% in after-hours trading. Investors reacted positively to the company’s long-term strategy and growth forecasts, despite short-term concerns over product-specific revenue trends.".to_string(),
                    },
                ]).with_graphics(
                vec![
                    Graphic {
                        text_id: "a".to_string(),
                        graph_type: "line".to_string(),
                        data_used: r#"[{"x": 1, "y": 2}, {"x": 2, "y": 3}, {"x": 3, "y": 4}, {"x": 4, "y": 5}, {"x": 5, "y": 6}, 
                          {"x": 6, "y": 7}, {"x": 7, "y": 8}, {"x": 8, "y": 9}, {"x": 9, "y": 10}, {"x": 10, "y": 11}]"#.to_string(),
                        purpose: "Revenue trends".to_string(),
                        x_label: "Quarter".to_string(),
                        y_label: "Revenue".to_string(),
                    },
                    Graphic {
                        text_id: "b".to_string(),
                        graph_type: "table".to_string(),
                        data_used: r#"{"rows": [["Q1", "Q2", "Q3", "Q4", "Q5", "Q6", "Q7", "Q8"], 
                                     ["$100M", "$200M", "$300M", "$400M", "$500M", "$600M", "$700M", "$800M"]], 
                          "columns": ["Q1", "Q2", "Q3", "Q4", "Q5", "Q6", "Q7", "Q8"]}"#.to_string(),
                        purpose: "Revenue comparison".to_string(),
                        x_label: "Quarter".to_string(),
                        y_label: "Revenue".to_string(),
                    },
                    Graphic {
                        text_id: "c".to_string(),
                        graph_type: "bar".to_string(),
                        data_used: r#"[{"category": "Product A", "value": 500}, {"category": "Product B", "value": 700}, 
                          {"category": "Product C", "value": 800}, {"category": "Product D", "value": 600}, 
                          {"category": "Product E", "value": 900}]"#.to_string(),
                        purpose: "Product sales distribution".to_string(),
                        x_label: "Product".to_string(),
                        y_label: "Sales".to_string(),
                    }
                ]
            )
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.charts.unwrap());
        dbg!(state.state.tables.unwrap());
    }
}
