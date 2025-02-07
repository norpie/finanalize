use crate::prelude::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod csv;
pub mod excel;
pub mod text;

#[async_trait]
trait ContentExtract {
    async fn extract(&self, file: &str) -> Result<Vec<String>>;
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
