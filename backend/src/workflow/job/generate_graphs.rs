use crate::llm::API;
use crate::prelude::*;
use crate::tasks::{Task, TaskResult};
use crate::workflow::job::generate_graphs::models::{
    BarDataOutput, LineDataOutput, PieDataOutput, StockDataOutput, TableDataOutput,
};
use crate::workflow::job::Job;
use crate::workflow::WorkflowState;
use crate::{graphing, prompting};
use async_trait::async_trait;
use log::debug;
use schemars::schema_for;

pub struct GenerateGraphsJob;

pub mod models {
    use crate::graphing::{GraphData, HistogramData, PieChartData, StockChartData};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Input {
        pub input: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
    pub struct GraphFileOutput {
        pub graph_caption: String,
        pub graph_type: String,
        pub file_path: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
    pub struct LineDataOutput {
        pub graph_data: GraphData,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
    pub struct BarDataOutput {
        pub graph_data: HistogramData,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
    pub struct PieDataOutput {
        pub graph_data: PieChartData,
    }
    #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
    pub struct StockDataOutput {
        pub graph_data: StockChartData,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
    pub struct TableDataOutput {
        pub graph_data: TableOutput,
    }
    #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
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
        let prompt = prompting::get_prompt("graph-data-prep".to_string())?;
        let task = Task::new(&prompt);
        for visual in state.state.visuals.clone().unwrap() {
            let task = task.clone();
            let input = models::Input {
                input: serde_json::to_string_pretty(&visual)?,
            };
            dbg!(&input);
            debug!("Prepared input: {:#?}", input);
            debug!("Running task...");
            match visual.visual_type.as_str() {
                "line" => {
                    let res: TaskResult<LineDataOutput> = task
                        .run_structured(
                            API.clone(),
                            &input,
                            serde_json::to_string_pretty(&schema_for!(LineDataOutput))?,
                        )
                        .await?;
                    let output = res.output;
                    state.state.generation_results.push(res.info);
                    let chart = graphing::create_graph(
                        "line".to_string(),
                        Some(output.graph_data),
                        None,
                        None,
                        None,
                    )?;
                    let graph_file_output = models::GraphFileOutput {
                        graph_caption: chart.chart_caption.clone(),
                        graph_type: chart.chart_type.clone(),
                        file_path: chart.chart_file.clone(),
                    };
                    charts.push(graph_file_output);
                }
                "bar" => {
                    let output: BarDataOutput = task
                        .run_structured(
                            API.clone(),
                            &input,
                            serde_json::to_string_pretty(&schema_for!(BarDataOutput))?,
                        )
                        .await?;
                    let chart = graphing::create_graph(
                        "bar".to_string(),
                        None,
                        Some(output.graph_data),
                        None,
                        None,
                    )
                    .expect("Could not create histogram");
                    let graph_file_output = models::GraphFileOutput {
                        graph_caption: chart.chart_caption.clone(),
                        graph_type: chart.chart_type.clone(),
                        file_path: chart.chart_file.clone(),
                    };
                    charts.push(graph_file_output);
                }
                "pie" => {
                    let output: PieDataOutput = task
                        .run_structured(
                            API.clone(),
                            &input,
                            serde_json::to_string_pretty(&schema_for!(PieDataOutput))?,
                        )
                        .await?;
                    let chart = graphing::create_graph(
                        "pie".to_string(),
                        None,
                        None,
                        Some(output.graph_data),
                        None,
                    )?;
                    let graph_file_output = models::GraphFileOutput {
                        graph_caption: chart.chart_caption.clone(),
                        graph_type: chart.chart_type.clone(),
                        file_path: chart.chart_file.clone(),
                    };
                    charts.push(graph_file_output);
                }
                "stock" => {
                    let output: StockDataOutput = task
                        .run_structured(
                            API.clone(),
                            &input,
                            serde_json::to_string_pretty(&schema_for!(StockDataOutput))?,
                        )
                        .await?;
                    let chart = graphing::create_graph(
                        "stock".to_string(),
                        None,
                        None,
                        None,
                        Some(output.graph_data),
                    )?;

                    let graph_file_output = models::GraphFileOutput {
                        graph_caption: chart.chart_caption.clone(),
                        graph_type: chart.chart_type.clone(),
                        file_path: chart.chart_file.clone(),
                    };
                    charts.push(graph_file_output);
                }
                "table" => {
                    let output: TableDataOutput = task
                        .run_structured(
                            API.clone(),
                            &input,
                            serde_json::to_string_pretty(&schema_for!(TableDataOutput))?,
                        )
                        .await?;
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
        debug!("GenerateGraphsJob completed");
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::extractors::{Column, Data};
    use crate::workflow::job::generate_visualizations::models::Visualization;
    use crate::{
        models::FullReport,
        workflow::{JobType, WorkflowState},
    };

    #[tokio::test]
    #[ignore = "Uses LLM API (External Service)"]
    async fn test_generate_graphs_job() {
        env_logger::init();
        dotenvy::from_filename(".env").ok();
        let job = GenerateGraphsJob;
        let state = WorkflowState {
            id: "tlksajbdfaln".into(),
            last_job_type: JobType::Pending,
            state: FullReport::new("sjaudnhcrlas".into(), "Apple stock in 2025".into())
                .with_title("State of Apple in 2025".into())
                .with_visuals(vec![
                    Visualization {
                        visual_type: "bar".into(),
                        data: Data {
                            title: "Apple Revenue and Profit in 2025".into(),
                            description: "Comparison of Apple's revenue and profit for Q1, Q2, Q3, and Q4 in 2025.".into(),
                            columns: vec![
                                Column {
                                    name: "Quarter".into(),
                                    description: "Financial Quarters of 2025".into(),
                                    values: vec!["Q1".into(), "Q2".into(), "Q3".into(), "Q4".into()],
                                },
                                Column {
                                    name: "Revenue".into(),
                                    description: "Apple's Revenue (in billions USD)".into(),
                                    values: vec!["110".into(), "120".into(), "130".into(), "140".into()],
                                },
                                Column {
                                    name: "Profit".into(),
                                    description: "Apple's Profit (in billions USD)".into(),
                                    values: vec!["30".into(), "32".into(), "34".into(), "36".into()],
                                },
                            ],
                        },
                    },
                    Visualization {
                        visual_type: "line".into(),
                        data: Data {
                            title: "Apple Stock Price Trends in 2025".into(),
                            description: "Monthly average stock prices of Apple throughout 2025.".into(),
                            columns: vec![
                                Column {
                                    name: "Month".into(),
                                    description: "Months of the year 2025".into(),
                                    values: vec![
                                        "1.0".into(), "2.0".into(), "3.0".into(), "4.0".into(), "5.0".into(), "6.0".into(),
                                        "7.0".into(), "8.0".into(), "9.0".into(), "10.0".into(), "11.0".into(), "12.0".into(),
                                    ],
                                },
                                Column {
                                    name: "Stock Price".into(),
                                    description: "Average Stock Price (in USD)".into(),
                                    values: vec![
                                        "150".into(), "155".into(), "160".into(), "158".into(),
                                        "162".into(), "170".into(), "175".into(), "180".into(),
                                        "178".into(), "185".into(), "190".into(), "195".into(),
                                    ],
                                },
                            ],
                        },
                    },
                ]),
        };
        let state = job.run(state).await.unwrap();
        dbg!(&state.state.charts.unwrap());
        dbg!(&state.state.tables.unwrap());
    }
}
