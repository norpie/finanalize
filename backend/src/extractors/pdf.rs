use tokio::fs::File;
use tokio::io::AsyncReadExt;
use lopdf::{Document, Object};
use crate::prelude::*;

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

        Ok(Self::split_text(&extracted_text))
    }

    fn split_text(text: &str) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        for word in text.split_whitespace() {
            if current_chunk.len() + word.len() >= 512 {
                chunks.push(current_chunk.clone());
                current_chunk.clear();
            }
            if !current_chunk.is_empty() {
                current_chunk.push(' ');
            }
            current_chunk.push_str(word);
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        chunks
    }
}








