use crate::prelude::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// pub mod csv;
pub mod html;
pub mod text;
pub mod md;
// pub mod figure;
pub mod pdf;

#[async_trait]
trait ContentExtract {
    async fn extract(&self, file_type: FileType) -> Result<Vec<Content>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Content {
    Html(String),
    Text(String),
    // Figures(Vec<Figure>),
    // Csv(String),
    MarkDown(String),
    Pdf(Vec<u8>),
    
}
#[derive(Debug, Serialize, Deserialize)]
pub enum FileType {
    Html(String),
    Text(String),
    // Figures(Vec<Figure>),
    // Csv(String),
    MarkDown(String),
    Pdf(Vec<u8>),
}

// #[derive(Debug, Serialize, Clone, Deserialize)]
// pub struct Figure {
//     pub url: String,
//     pub alt_text: Option<String>,
//     pub caption: Option<String>,
// }

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Data {
    title: String,
    description: String,
    columns: Vec<Column>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Column {
    name: String,
    description: String,
    values: Vec<String>,
}

#[async_trait]
trait DataExtract {
    async fn extract(&self, file: &str) -> Result<Data>;
}
