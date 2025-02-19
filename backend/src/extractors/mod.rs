use crate::prelude::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod csv;
pub mod html;
pub mod pdf;
pub mod text;

#[async_trait]
trait ContentExtract {
    async fn extract(&self, file: &str) -> Result<Vec<Content>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Content {
    Html(String),
    Text(String),
    Pdf(Vec<u8>),
    Csv(String),
    
}


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
