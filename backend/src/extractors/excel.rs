use crate::prelude::*;
use async_trait::async_trait;
use calamine::{open_workbook, Reader, Xlsx};
use std::io::Write;
use tempfile::NamedTempFile;

use super::{csv::CsvExtractor, Data, DataExtract};

pub struct ExcelExtractor;

#[async_trait]
impl DataExtract for ExcelExtractor {
    async fn extract(&self, file_path: &str) -> Result<Data> {
        // Convert Excel to CSV
        let mut workbook: Xlsx<_> = open_workbook(file_path)?;
        // TODO: Fix error
        let sheet_names = workbook.sheet_names();
        let sheet_name = sheet_names
            .first()
            .ok_or(FinanalizeError::InternalServerError)?;
        let range = workbook.worksheet_range(sheet_name)?;

        // Create a temporary CSV file
        let mut temp_csv = NamedTempFile::new()?;
        for row in range.rows() {
            let row_str = row
                .iter()
                .map(|cell| cell.to_string())
                .collect::<Vec<_>>()
                .join(",");
            writeln!(temp_csv, "{}", row_str)?;
        }

        // Use CsvExtractor to process the CSV file
        let csv_extractor = CsvExtractor;
        csv_extractor
            .extract(temp_csv.path().to_str().unwrap())
            .await
    }
}
//
// #[cfg(test)]
// mod tests {
//     use std::fs::File;
//
//     use super::*;
//     use tempfile::tempdir;
//
//     #[tokio::test]
//     async fn test_extract() {
//         // Create a temporary directory
//         let dir = tempdir().unwrap();
//         let file_path = dir.path().join("sample.xlsx");
//
//         // Write sample Excel data to the file
//         let mut workbook =
//             calamine::Xlsx::<std::fs::File>::new(File::create(&file_path).unwrap()).unwrap();
//         let mut sheet = workbook.new_sheet("Sheet1").unwrap();
//         sheet.write_string(0, 0, "name").unwrap();
//         sheet.write_string(0, 1, "age").unwrap();
//         sheet.write_string(0, 2, "city").unwrap();
//         sheet.write_string(1, 0, "Alice").unwrap();
//         sheet.write_string(1, 1, "30").unwrap();
//         sheet.write_string(1, 2, "New York").unwrap();
//         sheet.write_string(2, 0, "Bob").unwrap();
//         sheet.write_string(2, 1, "25").unwrap();
//         sheet.write_string(2, 2, "Los Angeles").unwrap();
//         sheet.write_string(3, 0, "Charlie").unwrap();
//         sheet.write_string(3, 1, "35").unwrap();
//         sheet.write_string(3, 2, "Chicago").unwrap();
//         workbook.close().unwrap();
//
//         // Create an instance of ExcelExtractor
//         let extractor = ExcelExtractor;
//
//         // Call the extract function
//         let result = extractor.extract(file_path.to_str().unwrap()).await;
//
//         // Assert that the result is Ok
//         assert!(result.is_ok());
//
//         // Clean up the temporary directory
//         dir.close().unwrap();
//     }
// }
