use super::{Column, Data, DataExtract};
use crate::workflow::job::classify_sources::models::ClassifySourcesInput;
use crate::{llm::API, prelude::*, prompting, tasks::Task};
use async_trait::async_trait;
use log::debug;
use polars::{io::SerReader, prelude::CsvReadOptions};
use serde::{Deserialize, Serialize};

pub struct CsvExtractor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataClassifierOuput {
    pub title: String,
    pub description: String,
    pub columns: Vec<ClassifierColumn>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ClassifierColumn {
    pub title: String,
    pub description: String,
}

#[async_trait]
impl DataExtract for CsvExtractor {
    async fn extract(&self, file: &str) -> Result<Data> {
        let df = CsvReadOptions::default()
            .try_into_reader_with_file_path(Some(file.into()))?
            .finish()?;

        // TODO: Make head into markdown table
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
        let output: DataClassifierOuput = task.run_structured(API.clone(), &input).await?;

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
        debug!("Columns: {:?}", columns);
        // TODO: Generate actual title and description from DataFrame metadata

        //Return the output

        let data = Data {
            title: output.title,
            description: output.description,
            columns,
        };
        debug!("Data: {:?}", data);
        // Return as Vec<Content> by converting Data into Content::Csv
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[tokio::test]
    #[ignore = "Uses Ollama for generation"]
    async fn test_extract() {
        dotenvy::from_filename(".env").ok();
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("sample.csv");
        let csv_file = file_path.to_str().unwrap();

        // Write sample CSV data to the file
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "name,age,city").unwrap();
        writeln!(file, "Alice,30,New York").unwrap();
        writeln!(file, "Bob,25,Los Angeles").unwrap();
        writeln!(file, "Charlie,35,Chicago").unwrap();

        // Create an instance of CsvExtractor
        let extractor = CsvExtractor;

        // Call the extract function
        let result = extractor.extract(csv_file).await;

        // Assert that the result is Ok
        let content = result.unwrap();

        // Make sure we have at least one content in the result (for CSV)
        assert!(!content.columns.is_empty());
        dbg!(content);

        // Clean up the temporary directory
        dir.close().unwrap();
    }
}
