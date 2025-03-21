use crate::llm::API;
use crate::prelude::*;
use crate::prompting;
use crate::tasks::Task;
use crate::workflow::job::graph_identifier::models::GraphIdentifierOutput;
use crate::workflow::job::Job;
use crate::workflow::WorkflowState;
use async_trait::async_trait;
use log::debug;
use schemars::schema_for;

pub struct GraphIdentifierJob;
pub mod models {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GraphIdentifierInput {
        pub report_text: String,
        pub chart_caption: Option<String>,
        pub table_caption: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    pub struct GraphIdentifierOutput {
        pub chart_caption: Option<String>,
        pub table_caption: Option<String>,
        pub position: Vec<String>,
    }
}

#[async_trait]
impl Job for GraphIdentifierJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        debug!("Running GraphIdentifierJob...");
        let sub_section_contents = state.state.sub_section_contents.clone().unwrap();
        let mut chart_positions = Vec::new();
        // let mut table_positions = Vec::new();
        let prompt = prompting::get_prompt("graph-identifier".into())?;
        let task = Task::new(&prompt);
        let charts = state.state.charts.clone().unwrap();
        // let tables = state.state.tables.clone().unwrap();
        for section in sub_section_contents {
            for sub_section_content in section.clone() {
                for chart in &charts {
                    let task = task.clone();
                    let input = models::GraphIdentifierInput {
                        report_text: sub_section_content.clone(),
                        chart_caption: Some(chart.graph_caption.clone()),
                        table_caption: None,
                    };
                    debug!("Prepared input: {:#?}", input);
                    debug!("Running task...");
                    let output: GraphIdentifierOutput = task
                        .run_structured(
                            API.clone(),
                            &input,
                            serde_json::to_string_pretty(&schema_for!(GraphIdentifierOutput))?,
                        )
                        .await?;
                    chart_positions.push(output);
                }
                // debug!("Task completed");
                // for table in &tables {
                //     let input = models::GraphIdentifierInput {
                //         report_text: sub_section_content.clone(),
                //         chart_caption: None,
                //         table_caption: Some(table.caption.clone()),
                //     };
                //     debug!("Prepared input: {:#?}", input);
                //     debug!("Running task...");
                //     let output: GraphIdentifierOutput = task
                //         .run_structured(
                //             API.clone(),
                //             &input,
                //             serde_json::to_string_pretty(&schema_for!(GraphIdentifierOutput))?,
                //         )
                //         .await?;
                //     table_positions.push(output);
                // }
                debug!("Task completed");
            }
        }

        state.state.chart_positions = Some(chart_positions);
        // state.state.table_positions = Some(table_positions);
        debug!("GraphInsertionJob completed.");
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow::job::generate_graphs::models::{GraphFileOutput, TableOutput};
    use crate::{
        models::FullReport,
        workflow::{JobType, WorkflowState},
    };

    #[tokio::test]
    #[ignore = "Uses LLM API (External Service)"]
    async fn test_identify_graphs_job() {
        env_logger::init();
        dotenvy::from_filename(".env").ok();
        let job = GraphIdentifierJob;
        let state = WorkflowState {
            id: "tlksajbdfaln".into(),
            last_job_type: JobType::Pending,
            state: FullReport::new("sjaudnhcrlas".into(), "Apple stock in 2025".to_string())
                .with_title("State of Apple in 2025".into())
                .with_report_text("In 2025, Apple continues to be a dominant force in the tech industry.
                The company's stock has shown steady growth, driven by strong product sales and innovations
                in artificial intelligence and augmented reality. Analysts predict that Apple's stock price
                will reach new highs as demand for its devices remains strong.
                Recent reports indicate that Apple has expanded its services sector, contributing
                significantly to its revenue. With new product launches and a growing customer base,
                Apple remains a key player in the global market.".to_string())
                .with_charts(vec![GraphFileOutput{
                    graph_caption: "Apple stock price in 2025".to_string(),
                    graph_type: "stock".to_string(),
                    file_path: "https://example.com/apple_stock_2025.png".to_string()
                }])
                .with_tables(vec![TableOutput{
                    caption: "Apple stock price evolution between 2024 and 2025".to_string(),
                    rows: vec![
                        vec!["2024".to_string(), "$150".to_string()],
                        vec!["2025".to_string(), "$175".to_string()],
                    ],
                    columns: vec!["Year".to_string(), "Stock Price".to_string()]
                }])
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.chart_positions.unwrap());
        dbg!(state.state.table_positions.unwrap());
    }
}
