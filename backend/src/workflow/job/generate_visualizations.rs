use crate::llm::API;
use crate::prelude::*;
use crate::prompting;
use crate::tasks::Task;
use crate::workflow::job::generate_visualizations::models::{
    ColumnInput, DataInput, Visualization, VisualizationOutput,
};
use crate::workflow::job::Job;
use crate::workflow::WorkflowState;
use async_trait::async_trait;
use log::debug;
use schemars::schema_for;
pub struct GenerateVisualizationsJob;
pub mod models {
    use crate::extractors::Data;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct DataInput {
        pub title: String,
        pub description: String,
        pub columns: Vec<ColumnInput>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ColumnInput {
        pub name: String,
        pub description: String,
    }
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct VisualizationInput {
        pub data: DataInput,
        pub graph_types: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
    pub struct Visualization {
        pub visual_type: String,
        pub data: Data,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
    pub struct VisualizationOutput {
        pub visual_type: String,
    }
}

#[async_trait]
impl Job for GenerateVisualizationsJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        debug!("Running GenerateVisualizationsJob...");
        let mut visuals = Vec::new();
        if let Some(extracted_data) = state.state.extracted_data.clone() {
            for data in extracted_data {
                let data_input = DataInput {
                    title: data.title.clone(),
                    description: data.description.clone(),
                    columns: data
                        .columns
                        .iter()
                        .map(|c| ColumnInput {
                            name: c.name.clone(),
                            description: c.description.clone(),
                        })
                        .collect(),
                };
                let input = models::VisualizationInput {
                    data: data_input.clone(),
                    graph_types: vec![
                        "bar".to_string(),
                        "line".to_string(),
                        "pie".to_string(),
                        "stock".to_string(),
                        "table".to_string(),
                    ],
                };
                debug!("Prepared input: {:#?}", input);
                debug!("Running task...");
                let prompt = prompting::get_prompt("graph-visualization".into())?;
                let task = Task::new(&prompt);
                let output: VisualizationOutput = task
                    .run_structured(
                        API.clone(),
                        &input,
                        serde_json::to_string_pretty(&schema_for!(Visualization))?,
                    )
                    .await?;
                visuals.push(Visualization {
                    visual_type: output.visual_type.clone(),
                    data: data.clone(),
                });
                debug!("Task completed");
            }
            state.state.visuals = Some(visuals);
            debug!("Visualizations: {:#?}", state.state.visuals);
            dbg!(&state.state.visuals);
        }
        debug!("GenerateVisualizationsJob completed");
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extractors::{Column, Data};
    use crate::{
        models::FullReport,
        workflow::{JobType, WorkflowState},
    };

    #[tokio::test]
    #[ignore = "Uses LLM API (External Service)"]
    async fn test_generate_visuals_job() {
        env_logger::init();
        dotenvy::from_filename(".env").ok();
        let job = GenerateVisualizationsJob;
        let state = WorkflowState {
            id: "tlksajbdfaln".into(),
            last_job_type: JobType::Pending,
            state: FullReport::new("sjaudnhcrlas".into(), "Apple stock in 2025".into())
                .with_title("State of Apple in 2025".into())
                .with_extracted_data(vec![
                    Data {
                        title: "Stock prices for Apple between 2020 and 2025".into(),
                        description:
                            "This data contains the stock prices for Apple between 2020 and 2025."
                                .into(),
                        columns: vec![
                            Column {
                                name: "Year".to_string(),
                                description: "The year of the stock price".to_string(),
                                values: vec![
                                    "2020".to_string(),
                                    "2021".to_string(),
                                    "2022".to_string(),
                                    "2023".to_string(),
                                    "2024".to_string(),
                                    "2025".to_string(),
                                ],
                            },
                            Column {
                                name: "Stock Price".to_string(),
                                description: "Closing stock price at the end of the year"
                                    .to_string(),
                                values: vec![
                                    "$120".to_string(),
                                    "$135".to_string(),
                                    "$150".to_string(),
                                    "$160".to_string(),
                                    "$175".to_string(),
                                    "$190".to_string(),
                                ],
                            },
                        ],
                    },
                    Data {
                        title: "Apple Revenue from 2020 to 2025".into(),
                        description:
                            "This dataset shows the annual revenue of Apple from 2020 to 2025."
                                .into(),
                        columns: vec![
                            Column {
                                name: "Year".to_string(),
                                description: "The fiscal year of the reported revenue".to_string(),
                                values: vec![
                                    "2020".to_string(),
                                    "2021".to_string(),
                                    "2022".to_string(),
                                    "2023".to_string(),
                                    "2024".to_string(),
                                    "2025".to_string(),
                                ],
                            },
                            Column {
                                name: "Revenue (Billion $)".to_string(),
                                description: "Total revenue in billion USD".to_string(),
                                values: vec![
                                    "260".to_string(),
                                    "275".to_string(),
                                    "300".to_string(),
                                    "320".to_string(),
                                    "350".to_string(),
                                    "380".to_string(),
                                ],
                            },
                        ],
                    },
                ]),
        };
        let state = job.run(state).await.unwrap();
        dbg!(state.state.visuals.unwrap());
    }
}
