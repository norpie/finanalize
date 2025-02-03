use async_trait::async_trait;
use crate::prelude::*;
pub mod csv;

#[async_trait]
trait Extract {
    async fn extract(&self, file: &str) -> Result<()>;

}
