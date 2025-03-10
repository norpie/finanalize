use crate::prelude::*;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod csv;
pub mod figure;
pub mod html;
pub mod md;
pub mod pdf;
pub mod text;

#[async_trait]
pub trait ContentExtract {
    async fn extract(&self, file_type: FileType) -> Result<Vec<Content>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Content {
    Html(String),
    Text(String),
    Figures(Vec<Figure>),
    Csv(String),
    MarkDown(String),
    Pdf(Vec<u8>),
}
#[derive(Debug, Serialize, Deserialize)]
pub enum FileType {
    Html(String),
    Text(String),
    Figures(Vec<Figure>),
    Csv(String),
    MarkDown(String),
    Pdf(Vec<u8>),
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Figure {
    pub url: String,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Data {
    pub title: String,
    pub description: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Column {
    pub name: String,
    pub description: String,
    pub values: Vec<String>,
}

#[async_trait]
trait DataExtract {
    async fn extract(&self, file: &str) -> Result<Data>;
}
