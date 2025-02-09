use crate::prelude::*;
use async_trait::async_trait;
use polars::{io::SerReader, prelude::CsvReadOptions};

use super::{Column, Data, DataExtract};

pub struct CsvExtractor;

#[async_trait]
impl DataExtract for CsvExtractor {
    async fn extract(&self, file_path: &str) -> Result<Data> {
        let df = CsvReadOptions::default()
            .try_into_reader_with_file_path(Some(file_path.into()))?
            .finish()?;
        let mut columns = vec![];
        for column in df.get_columns() {
            let column = column.cast(&polars::prelude::DataType::String)?;
            columns.push(Column {
                name: column.name().as_str().into(),
                description: String::new(),
                values: column
                    .str()?
                    .into_iter()
                    .map(|v| v.unwrap_or("").into())
                    .collect(),
            });
        }

        // TODO: Use to generate the real stuff
        let title = "CSV Data Analysis".to_string();
        let description = format!(
            "DataFrame with {} rows and {} columns",
            df.height(),
            df.width()
        );
        let _head = df.head(Some(5));

        Ok(Data {
            title,
            description,
            columns,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_extract() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("sample.csv");

        // Write sample CSV data to the file
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "name,age,city").unwrap();
        writeln!(file, "Alice,30,New York").unwrap();
        writeln!(file, "Bob,25,Los Angeles").unwrap();
        writeln!(file, "Charlie,35,Chicago").unwrap();

        // Create an instance of CsvExtractor
        let extractor = CsvExtractor;

        // Call the extract function
        let result = extractor.extract(file_path.to_str().unwrap()).await;

        // Assert that the result is Ok
        result.unwrap();

        // Clean up the temporary directory
        dir.close().unwrap();
    }
}
