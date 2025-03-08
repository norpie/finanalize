use crate::workflow::job::graphic_identifier::models::Graphic;
use crate::workflow::job::Job;
use crate::workflow::WorkflowState;
use crate::{llm::API, prelude::*, prompting, tasks::Task};
use async_trait::async_trait;
use log::debug;
use models::GraphDataOutput;
use schemars::schema_for;

pub mod models {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct GraphInput {
        pub chart_options: Vec<String>,
        pub text: String,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
    pub struct GraphDataOutput {
        pub graphics: Vec<GraphicOutput>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Text {
        pub id: String,
        pub text: String,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
    pub struct GraphicOutput {
        pub graph_type: String,
        pub purpose: String,
        pub data_used: String,
        pub x_label: String,
        pub y_label: String,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Graphic {
        pub text_id: String,
        pub graph_type: String,
        pub purpose: String,
        pub data_used: String,
        pub x_label: String,
        pub y_label: String,
    }
}
pub struct GraphIdentifierJob;

#[async_trait]
impl Job for GraphIdentifierJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        debug!("Running GraphIdentifierJob...");
        let mut graphics = Vec::new();
        if let Some(texts) = state.state.texts.clone() {
            for text in texts {
                let prompt = prompting::get_prompt("graph-identifier".into())?;
                let task = Task::new(&prompt);
                let input = models::GraphInput {
                    chart_options: vec![
                        "table".to_string(),
                        "line".to_string(),
                        "bar".to_string(),
                        "pie".to_string(),
                        "stock".to_string(),
                    ],
                    text: text.text.clone(),
                };
                debug!("Prepared input: {:#?}", input);
                debug!("Running task...");
                let output: GraphDataOutput = task
                    .run_structured(
                        API.clone(),
                        &input,
                        serde_json::to_string_pretty(&schema_for!(GraphDataOutput))?,
                    )
                    .await?;
                graphics.extend(output.graphics.into_iter().map(|g| Graphic {
                    text_id: text.id.clone(),
                    graph_type: g.graph_type,
                    purpose: g.purpose,
                    data_used: g.data_used,
                    x_label: g.x_label,
                    y_label: g.y_label,
                }));
                debug!("Task completed");
            }
            state.state.graphics = Some(graphics);
            debug!("Graph: {:#?}", state.state.report);
            dbg!(&state.state.report);
        }
        debug!("GraphIdentifierJob completed");
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::models::PreClassificationSource;
    use crate::workflow::job::graphic_identifier::models::Text;
    use crate::{
        models::FullReport,
        workflow::{JobType, WorkflowState},
    };

    #[tokio::test]
    #[ignore = "Uses LLM API (External Service)"]
    async fn test_graphic_identifier_job() {
        env_logger::init();
        dotenvy::from_filename(".env").ok();
        let job = GraphIdentifierJob;
        let state = WorkflowState {
            id: "tlksajbdfaln".into(),
            last_job_type: JobType::Pending,
            state: FullReport::new("sjaudnhcrlas".into(), "Apple stock in 2025".into())
                .with_title("State of Apple in 2025".into())
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
                ]),
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.graphics.unwrap());
    }
}
