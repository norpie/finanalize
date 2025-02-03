use crate::prelude::*;
use crate::extract::Extract; // Import the Extract trait
use async_trait::async_trait;
use polars::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub struct CsvExtractor;

#[async_trait]
impl Extract for CsvExtractor {
    async fn extract(&self, file_path: &str) -> crate::prelude::Result<()> { // Disambiguate Result
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let df = CsvReader::new(reader)
            .infer_schema(None)
            .has_header(true)
            .finish()?;

        // Extract head of the CSV file
        let head = df.head(Some(5));

        // Generate description,title, and possible graphs
        let description = format!("DataFrame with {} rows and {} columns", df.height(), df.width());
        let title = "CSV Data Analysis";
        let possible_graphs = vec!["Histogram", "Scatter Plot", "Line Chart"];

        // Save to database (pseudo-code, replace with actual database logic)
        // save_to_database(description, title, possible_graphs);

        Ok(())
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
        assert!(result.is_ok());

        // Clean up the temporary directory
        dir.close().unwrap();
    }
}