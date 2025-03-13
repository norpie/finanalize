use std::path::PathBuf;
use std::str::FromStr;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::prelude::*;

use crate::workflow::WorkflowState;

use super::Job;

pub struct GeneratePreviewDocumentJob;

#[async_trait]
impl Job for GeneratePreviewDocumentJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let full_pdf_path = PathBuf::from_str(&state.state.report.clone().unwrap()).unwrap();
        let mut pdf = lopdf::Document::load(full_pdf_path)?;
        // Delete the pages after the 5th page
        let pages: u32 = pdf.get_pages().len().try_into().unwrap();
        let page_numbers: Vec<u32> = (5..pages+1).collect();
        pdf.delete_pages(&page_numbers);
        // save new pdf
        let dir = format!("/tmp/{}", Uuid::new_v4());
        let dir_path = PathBuf::from_str(&dir).unwrap();
        std::fs::create_dir_all(&dir_path)?;
        let new_pdf_path = dir_path.join("preview.pdf");
        pdf.save(&new_pdf_path)?;
        let str_path = new_pdf_path.to_str().unwrap();
        debug!("Preview document generated: {}", &str_path);
        state.state.preview = Some(str_path.to_string());
        Ok(state)
    }
}
