use std::io::Cursor;

use crate::{
    extractors::{csv::DataClassifierOuput, Column, Data},
    llm::API,
    prelude::*,
    prompting,
    tasks::Task,
    workflow::{job::classify_sources::models::ClassifySourcesInput, WorkflowState},
};

use async_trait::async_trait;
use log::debug;
use polars::{io::SerReader, prelude::CsvReader};
use schemars::schema_for;

use super::Job;

pub struct ClassifyDataJob;

#[async_trait]
impl Job for ClassifyDataJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let mut classified_sources = vec![];
        for csv in state.state.csv_sources.clone().unwrap() {
            let df = CsvReader::new(Cursor::new(csv)).finish()?;
            let mut markdown_table = String::new();

            // Generate the header row of the table
            markdown_table.push_str("| ");
            for col_name in df.get_column_names() {
                markdown_table.push_str(&format!("{} | ", col_name));
            }
            markdown_table.push('\n');

            // Generate the separator row
            markdown_table.push_str("| ");
            for _ in df.get_column_names() {
                markdown_table.push_str("--- | ");
            }
            markdown_table.push('\n');

            // Generate the data rows
            for i in 0..5 {
                markdown_table.push_str("| ");
                for col in df.get_columns() {
                    let value = col.get(i).unwrap_or_default().to_string();
                    markdown_table.push_str(&format!("{} | ", value));
                }
                markdown_table.push('\n');
            }

            //Use ClassifySourceInput and put the table in it
            let input = ClassifySourcesInput {
                input: markdown_table,
            };

            //Start job run structured data classification
            let prompt = prompting::get_prompt("data-classifier".into())?;
            let task = Task::new(&prompt);
            let output: DataClassifierOuput = task
                .run_structured(
                    API.clone(),
                    &input,
                    serde_json::to_string_pretty(&schema_for!(DataClassifierOuput))?,
                )
                .await?;

            // After getting your output from the task.run_structured call
            let mut columns = vec![];
            for column in df.get_columns() {
                let column = column.cast(&polars::prelude::DataType::String)?;
                let column_name = column.name().as_str();

                // Find the matching description from the classifier output
                let description = output
                    .columns
                    .iter()
                    .find(|c| c.title == column_name)
                    .map(|c| c.description.clone())
                    .unwrap_or_else(String::new);

                columns.push(Column {
                    name: column_name.into(),
                    description, // Use the found description instead of an empty string
                    values: column
                        .str()?
                        .into_iter()
                        .map(|v| v.unwrap_or("").into())
                        .collect(),
                });
            }
            let data = Data {
                title: output.title,
                description: output.description,
                columns,
            };
            classified_sources.push(data);
        }
        state.state.classified_data_sources = Some(classified_sources);
        Ok(state)
    }
}
