use crate::prelude::*;
use lopdf::Document;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use super::text::TextExtractor;
use super::ContentExtract;

pub struct PDFExtractor;

impl PDFExtractor {
    pub async fn extract(mut file: File) -> Result<Vec<String>> {
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .await
            .map_err(FinanalizeError::from)?;

        let doc = Document::load_mem(&buffer).map_err(FinanalizeError::from)?;

        let mut extracted_text = String::new();

        for (_, (page_id, _)) in doc.get_pages().iter() {
            if let Ok(text) = doc.extract_text(&[*page_id]) {
                extracted_text.push_str(&text);
                extracted_text.push('\n');
            }
        }

        if extracted_text.is_empty() {
            return Err(FinanalizeError::NotFound);
        }

        TextExtractor {}.extract(&extracted_text).await
    }
}
