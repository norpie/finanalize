use std::io::{BufWriter, Write};

use super::{html::HTMLExtractor, Content, ContentExtract, FileType};
use crate::prelude::*;
use async_trait::async_trait;
use log::debug;
use lopdf::Document;
use pdf_extract::HTMLOutput;
// Import Error for `lopdf`
use tokio::task;

pub struct PdfExtractor;

#[async_trait]
impl ContentExtract for PdfExtractor {
    async fn extract(&self, file: FileType) -> Result<Vec<Content>> {
        debug!("Extracting content from PDF...");
        let FileType::Pdf(buffer) = file else {
            return Err(FinanalizeError::ParseError(
                "Invalid input type".to_string(),
            ));
        };
        debug!("Valid PDF buffer received: {} bytes", buffer.len());

        // Perform PDF extraction in a blocking thread
        let html_content = task::spawn_blocking(move || {
            // Load the PDF document from the buffer using lopdf's `load_from` function
            let doc = Document::load_mem(&buffer)?;
            debug!("PDF document loaded successfully");
            // Create a buffer to write into
            let mut buffer = Vec::new();

            // Wrap the buffer with BufWriter
            {
                let mut writer = BufWriter::new(&mut buffer);
                let mut output = HTMLOutput::new(&mut writer);
                pdf_extract::output_doc(&doc, &mut output)?;
                writer.flush()?;
            }
            let output = String::from_utf8(buffer)?;
            debug!("Text extracted successfully");
            Ok::<_, FinanalizeError>(output) // Return the HTML-wrapped extracted text
        })
        .await??;

        let result = HTMLExtractor.extract(FileType::Html(html_content)).await?;
        debug!("Content extracted successfully from PDF: {} chunks", result.len());
        // Return the extracted HTML content wrapped in Content::Html
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pdf_extraction() {
        // Use the sample PDF data directly
        let sample_pdf_data = include_bytes!("../../tests/sample.pdf");

        let extractor = PdfExtractor;

        // Extract the content from the PDF
        let result = extractor
            .extract(FileType::Pdf(sample_pdf_data.to_vec()))
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
    }
}
